// This module manages HTTP connections with automatic protocol detection, keep-alive handling, and efficient buffer management.
// It coordinates between HTTP/1.1 and HTTP/2 parsers while maintaining connection state and enforcing timeouts for robust server operation.

use crate::server::{Protocol, ProtocolDetection, detect_protocol};
use crate::server::http1::{Http1Parser, Request as Http1Request};
use crate::server::http2::{Http2Parser, Frame as Http2Frame};
use crate::utils::pool::{BufferPool, PooledObject};
use crate::error::Error;
use std::io::{self, Read, Write};
use std::time::{Duration, Instant};
use std::net::SocketAddr;

/// Connection configuration
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub read_timeout: Duration,
    pub write_timeout: Duration,
    pub keep_alive_timeout: Duration,
    pub max_request_size: usize,
    pub max_header_size: usize,
    pub tcp_nodelay: bool,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
            keep_alive_timeout: Duration::from_secs(120),
            max_request_size: 10 * 1024 * 1024, // 10MB
            max_header_size: 8192,
            tcp_nodelay: true,
        }
    }
}

/// Connection state
#[derive(Debug)]
pub enum ConnectionState {
    /// Waiting for protocol detection
    Detecting,
    /// HTTP/1.1 connection
    Http1(Http1State),
    /// HTTP/2 connection
    Http2(Http2State),
    /// Connection is closing
    Closing,
}

/// HTTP/1.1 connection state
#[derive(Debug)]
pub struct Http1State {
    parser: Http1Parser,
    keep_alive: bool,
    requests_served: u64,
    pipeline_depth: usize,
}

/// HTTP/2 connection state
#[derive(Debug)]
pub struct Http2State {
    parser: Http2Parser,
    streams: crate::server::StreamManager,
    last_stream_id: u32,
}

/// Connection handler
pub struct Connection<S: Read + Write> {
    stream: S,
    peer_addr: SocketAddr,
    state: ConnectionState,
    config: ConnectionConfig,
    buffer_pool: BufferPool,
    read_buffer: PooledObject<Vec<u8>>,
    write_buffer: PooledObject<Vec<u8>>,
    created_at: Instant,
    last_activity: Instant,
    bytes_read: u64,
    bytes_written: u64,
}

impl<S: Read + Write> Connection<S> {
    /// I'm creating a new connection handler
    pub fn new(stream: S, peer_addr: SocketAddr, config: ConnectionConfig, buffer_pool: BufferPool) -> Self {
        let read_buffer = buffer_pool.get(8192);
        let write_buffer = buffer_pool.get(8192);
        
        Self {
            stream,
            peer_addr,
            state: ConnectionState::Detecting,
            config,
            buffer_pool,
            read_buffer,
            write_buffer,
            created_at: Instant::now(),
            last_activity: Instant::now(),
            bytes_read: 0,
            bytes_written: 0,
        }
    }

    /// Process the connection
    pub fn process(&mut self) -> Result<ConnectionAction, Error> {
        self.last_activity = Instant::now();

        match &mut self.state {
            ConnectionState::Detecting => self.detect_protocol(),
            ConnectionState::Http1(state) => self.process_http1(state),
            ConnectionState::Http2(state) => self.process_http2(state),
            ConnectionState::Closing => Ok(ConnectionAction::Close),
        }
    }

    /// Detect protocol from initial bytes
    fn detect_protocol(&mut self) -> Result<ConnectionAction, Error> {
        // Read initial data
        let bytes_read = self.read_available()?;
        if bytes_read == 0 {
            return Ok(ConnectionAction::NeedMore);
        }

        // Try to detect protocol
        if let Some(detection) = detect_protocol(&self.read_buffer[..bytes_read]) {
            match detection.protocol {
                Protocol::Http1 => {
                    self.state = ConnectionState::Http1(Http1State {
                        parser: Http1Parser::new(),
                        keep_alive: true,
                        requests_served: 0,
                        pipeline_depth: 0,
                    });
                    Ok(ConnectionAction::Continue)
                }
                Protocol::Http2 => {
                    self.state = ConnectionState::Http2(Http2State {
                        parser: Http2Parser::new(),
                        streams: crate::server::StreamManager::new(Some(100)),
                        last_stream_id: 0,
                    });
                    // Consume the preface
                    self.consume_read_buffer(detection.consumed);
                    Ok(ConnectionAction::Continue)
                }
                Protocol::Http3 => {
                    // HTTP/3 is over QUIC, not TCP
                    Err(Error::InvalidProtocol)
                }
            }
        } else {
            Ok(ConnectionAction::NeedMore)
        }
    }

    /// Process HTTP/1.1 connection
    fn process_http1(&mut self, state: &mut Http1State) -> Result<ConnectionAction, Error> {
        // Read more data if available
        let bytes_available = self.read_available()?;
        
        // Parse request
        match state.parser.parse_request(&self.read_buffer[..bytes_available]) {
            Ok((request, consumed)) => {
                self.consume_read_buffer(consumed);
                state.requests_served += 1;

                // Check for Connection header
                state.keep_alive = request.headers.iter()
                    .any(|h| h.name.eq_ignore_ascii_case("connection") && 
                             h.value.eq_ignore_ascii_case("keep-alive"));

                Ok(ConnectionAction::Request(HttpRequest::Http1(request)))
            }
            Err(crate::server::http1::Http1ParseError::IncompleteRequest) => {
                Ok(ConnectionAction::NeedMore)
            }
            Err(e) => Err(Error::ParseError(format!("HTTP/1.1 parse error: {}", e))),
        }
    }

    /// Process HTTP/2 connection
    fn process_http2(&mut self, state: &mut Http2State) -> Result<ConnectionAction, Error> {
        // Read more data if available
        let bytes_available = self.read_available()?;
        
        // Parse frame
        match state.parser.parse_frame(&self.read_buffer[..bytes_available]) {
            Ok((frame, consumed)) => {
                self.consume_read_buffer(consumed);
                
                // Handle frame based on type
                match frame.header.frame_type {
                    crate::server::http2::FrameType::Headers => {
                        // Create new stream
                        let stream_id = frame.header.stream_id;
                        if stream_id > state.last_stream_id {
                            state.last_stream_id = stream_id;
                            state.streams.create_stream()?;
                        }
                        
                        // Parse headers
                        let headers = state.parser.parse_headers(&frame.payload, frame.header.flags)?;
                        
                        Ok(ConnectionAction::Request(HttpRequest::Http2 {
                            stream_id,
                            headers,
                            body: None,
                        }))
                    }
                    crate::server::http2::FrameType::Data => {
                        Ok(ConnectionAction::Data {
                            stream_id: frame.header.stream_id,
                            data: frame.payload,
                            end_stream: frame.header.flags.has(crate::server::http2::FrameFlags::END_STREAM),
                        })
                    }
                    crate::server::http2::FrameType::Settings => {
                        let settings = state.parser.parse_settings(&frame.payload)?;
                        state.parser.update_settings(&settings)?;
                        
                        // Send settings ACK
                        self.send_settings_ack()?;
                        Ok(ConnectionAction::Continue)
                    }
                    _ => Ok(ConnectionAction::Continue),
                }
            }
            Err(crate::server::http2::Http2ParseError::IncompleteFrame) => {
                Ok(ConnectionAction::NeedMore)
            }
            Err(e) => Err(Error::ParseError(format!("HTTP/2 parse error: {:?}", e))),
        }
    }

    /// Read available data into buffer
    fn read_available(&mut self) -> Result<usize, Error> {
        let current_len = self.read_buffer.len();
        let capacity = self.read_buffer.capacity();
        
        if current_len >= capacity {
            // Buffer full, need to process data first
            return Ok(current_len);
        }

        // Ensure buffer has space
        self.read_buffer.resize(capacity, 0);
        
        match self.stream.read(&mut self.read_buffer[current_len..]) {
            Ok(0) => Err(Error::ConnectionClosed),
            Ok(n) => {
                self.read_buffer.truncate(current_len + n);
                self.bytes_read += n as u64;
                Ok(current_len + n)
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => Ok(current_len),
            Err(e) => Err(Error::Io(e)),
        }
    }

    /// Consume bytes from read buffer
    fn consume_read_buffer(&mut self, consumed: usize) {
        if consumed >= self.read_buffer.len() {
            self.read_buffer.clear();
        } else {
            self.read_buffer.drain(..consumed);
        }
    }

    /// Send HTTP/2 settings acknowledgment
    fn send_settings_ack(&mut self) -> Result<(), Error> {
        let frame = crate::server::http2::Http2FrameBuilder::new()
            .header(
                crate::server::http2::FrameType::Settings,
                0x1, // ACK flag
                0,
                0,
            )
            .build();
        
        self.write_all(&frame)
    }

    /// Write data to stream
    pub fn write_all(&mut self, data: &[u8]) -> Result<(), Error> {
        self.stream.write_all(data).map_err(Error::Io)?;
        self.bytes_written += data.len() as u64;
        Ok(())
    }

    /// Check if connection should be kept alive
    pub fn should_keep_alive(&self) -> bool {
        match &self.state {
            ConnectionState::Http1(state) => state.keep_alive,
            ConnectionState::Http2(_) => true, // HTTP/2 is always persistent
            _ => false,
        }
    }

    /// Check if connection has timed out
    pub fn is_timed_out(&self) -> bool {
        let timeout = match &self.state {
            ConnectionState::Detecting => self.config.read_timeout,
            _ => self.config.keep_alive_timeout,
        };
        
        self.last_activity.elapsed() > timeout
    }

    /// Get connection metrics
    pub fn metrics(&self) -> ConnectionMetrics {
        ConnectionMetrics {
            peer_addr: self.peer_addr,
            protocol: match &self.state {
                ConnectionState::Http1(_) => Some(Protocol::Http1),
                ConnectionState::Http2(_) => Some(Protocol::Http2),
                _ => None,
            },
            duration: self.created_at.elapsed(),
            bytes_read: self.bytes_read,
            bytes_written: self.bytes_written,
            requests_served: match &self.state {
                ConnectionState::Http1(state) => state.requests_served,
                ConnectionState::Http2(state) => state.streams.streams.len() as u64,
                _ => 0,
            },
        }
    }
}

/// Connection action to take
#[derive(Debug)]
pub enum ConnectionAction {
    /// Continue processing
    Continue,
    /// Need more data
    NeedMore,
    /// HTTP request received
    Request(HttpRequest),
    /// HTTP/2 data frame
    Data {
        stream_id: u32,
        data: Vec<u8>,
        end_stream: bool,
    },
    /// Close connection
    Close,
}

/// Unified HTTP request type
#[derive(Debug)]
pub enum HttpRequest {
    Http1(Http1Request<'static>),
    Http2 {
        stream_id: u32,
        headers: Vec<(Vec<u8>, Vec<u8>)>,
        body: Option<Vec<u8>>,
    },
}

/// Connection metrics
#[derive(Debug)]
pub struct ConnectionMetrics {
    pub peer_addr: SocketAddr,
    pub protocol: Option<Protocol>,
    pub duration: Duration,
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub requests_served: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    struct MockStream {
        read_data: Cursor<Vec<u8>>,
        write_data: Vec<u8>,
    }

    impl Read for MockStream {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.read_data.read(buf)
        }
    }

    impl Write for MockStream {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.write_data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_protocol_detection_http1() {
        let data = b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let stream = MockStream {
            read_data: Cursor::new(data.to_vec()),
            write_data: Vec::new(),
        };
        
        let addr = "127.0.0.1:8080".parse().unwrap();
        let mut conn = Connection::new(stream, addr, ConnectionConfig::default(), BufferPool::new());
        
        // First process detects protocol
        match conn.process().unwrap() {
            ConnectionAction::Continue => {},
            _ => panic!("Expected Continue"),
        }
        
        // Second process parses request
        match conn.process().unwrap() {
            ConnectionAction::Request(HttpRequest::Http1(req)) => {
                assert_eq!(req.method, crate::server::http1::Method::Get);
                assert_eq!(req.uri, "/");
            }
            _ => panic!("Expected HTTP/1.1 request"),
        }
    }

    #[test]
    fn test_connection_metrics() {
        let stream = MockStream {
            read_data: Cursor::new(vec![]),
            write_data: Vec::new(),
        };
        
        let addr = "127.0.0.1:8080".parse().unwrap();
        let conn = Connection::new(stream, addr, ConnectionConfig::default(), BufferPool::new());
        
        let metrics = conn.metrics();
        assert_eq!(metrics.peer_addr, addr);
        assert!(metrics.protocol.is_none());
        assert_eq!(metrics.bytes_read, 0);
        assert_eq!(metrics.bytes_written, 0);
    }

    #[test]
    fn test_timeout_detection() {
        let stream = MockStream {
            read_data: Cursor::new(vec![]),
            write_data: Vec::new(),
        };
        
        let mut config = ConnectionConfig::default();
        config.read_timeout = Duration::from_millis(10);
        
        let addr = "127.0.0.1:8080".parse().unwrap();
        let mut conn = Connection::new(stream, addr, config, BufferPool::new());
        
        assert!(!conn.is_timed_out());
        
        std::thread::sleep(Duration::from_millis(20));
        assert!(conn.is_timed_out());
    }
}
