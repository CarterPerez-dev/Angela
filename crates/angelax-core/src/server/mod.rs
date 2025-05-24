// This module provides high-performance HTTP/1.1 and HTTP/2 protocol implementations with SIMD-optimized parsing.
// It serves as the foundation for Angelax's protocol handling, offering zero-allocation parsing paths and exceptional throughput for both text-based HTTP/1.1 and binary HTTP/2 protocols.

pub mod http1;
pub mod http2;
pub mod connection;
pub mod tls;
pub mod http3;

use crate::error::Error;
use std::io;

/// Protocol version detected or negotiated
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Protocol {
    Http1,
    Http2,
    Http3,
}

/// Protocol detection result
pub struct ProtocolDetection {
    pub protocol: Protocol,
    pub consumed: usize,
}

//// Fast protocol detection using SIMD
pub fn detect_protocol(input: &[u8]) -> Option<ProtocolDetection> {
    // Check for HTTP/2 connection preface
    if input.len() >= 24 {
        if &input[..24] == http2::CONNECTION_PREFACE {
            return Some(ProtocolDetection {
                protocol: Protocol::Http2,
                consumed: 24,
            });
        }
    }

    // Check for HTTP/1.x request using SIMD when possible
    if input.len() >= 16 {
        // I'm using SIMD to quickly scan for HTTP method patterns
        if let Some(method_pos) = detect_http1_method_simd(input) {
            return Some(ProtocolDetection {
                protocol: Protocol::Http1,
                consumed: 0,
            });
        }
    }

    None
}

/// SIMD-accelerated HTTP/1.1 method detection
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
fn detect_http1_method_simd(input: &[u8]) -> Option<usize> {
    use std::arch::x86_64::*;
    
    unsafe {
        // I'm checking for common HTTP methods using SIMD
        // This checks for "GET ", "POST", "PUT ", "HEAD" in parallel
        let get_pattern = _mm_set_epi32(0, 0, 0, 0x20544547); // "GET "
        let post_pattern = _mm_set_epi32(0, 0, 0, 0x54534F50); // "POST"
        let put_pattern = _mm_set_epi32(0, 0, 0, 0x20545550); // "PUT "
        let head_pattern = _mm_set_epi32(0, 0, 0, 0x44414548); // "HEAD"
        
        if input.len() >= 4 {
            let data = _mm_loadu_si128(input.as_ptr() as *const __m128i);
            
            // Check all patterns
            if _mm_movemask_epi8(_mm_cmpeq_epi32(data, get_pattern)) != 0 ||
               _mm_movemask_epi8(_mm_cmpeq_epi32(data, post_pattern)) != 0 ||
               _mm_movemask_epi8(_mm_cmpeq_epi32(data, put_pattern)) != 0 ||
               _mm_movemask_epi8(_mm_cmpeq_epi32(data, head_pattern)) != 0 {
                return Some(0);
            }
        }
        
        // Check for longer methods
        let methods = [
            &b"DELETE "[..], &b"CONNECT "[..], &b"OPTIONS "[..],
            &b"TRACE "[..], &b"PATCH "[..],
        ];
        
        for method in methods {
            if input.starts_with(method) {
                return Some(0);
            }
        }
    }
    
    None
}

/// Fallback for non-SIMD platforms
#[cfg(not(all(target_arch = "x86_64", target_feature = "avx2")))]
fn detect_http1_method_simd(input: &[u8]) -> Option<usize> {
    // Fallback to simple byte comparison
    let methods = [
        &b"GET "[..], &b"HEAD "[..], &b"POST "[..], &b"PUT "[..],
        &b"DELETE "[..], &b"CONNECT "[..], &b"OPTIONS "[..],
        &b"TRACE "[..], &b"PATCH "[..],
    ];

    for method in methods {
        if input.starts_with(method) {
            return Some(0);
        }
    }
    
    None
}

/// Common trait for protocol parsers
pub trait ProtocolParser {
    type Request;
    type Response;
    type Error;

    /// Parse incoming data
    fn parse(&mut self, input: &[u8]) -> Result<(Self::Request, usize), Self::Error>;
    
    /// Build response
    fn build_response(&self, response: Self::Response) -> Vec<u8>;
}

/// Connection state for multiplexed protocols
#[derive(Debug)]
pub struct ConnectionState {
    pub protocol: Protocol,
    pub streams: StreamManager,
    pub flow_control: FlowController,
}

/// Stream manager for HTTP/2 and HTTP/3
#[derive(Debug)]
pub struct StreamManager {
    streams: std::collections::HashMap<u32, StreamState>,
    next_stream_id: u32,
    max_concurrent_streams: Option<u32>,
}

impl StreamManager {
    /// I'm creating a new stream manager
    pub fn new(max_concurrent: Option<u32>) -> Self {
        Self {
            streams: std::collections::HashMap::new(),
            next_stream_id: 1,
            max_concurrent_streams: max_concurrent,
        }
    }

    /// Create a new stream
    pub fn create_stream(&mut self) -> Result<u32, Error> {
        if let Some(max) = self.max_concurrent_streams {
            if self.streams.len() >= max as usize {
                return Err(Error::TooManyStreams);
            }
        }

        let stream_id = self.next_stream_id;
        self.next_stream_id += 2; // Client-initiated streams are odd

        self.streams.insert(stream_id, StreamState::Open);
        Ok(stream_id)
    }

    /// Get stream state
    pub fn get_stream(&self, stream_id: u32) -> Option<&StreamState> {
        self.streams.get(&stream_id)
    }

    /// Update stream state
    pub fn update_stream(&mut self, stream_id: u32, state: StreamState) {
        self.streams.insert(stream_id, state);
    }

    /// Remove closed streams
    pub fn cleanup_closed_streams(&mut self) {
        self.streams.retain(|_, state| !matches!(state, StreamState::Closed));
    }
}

/// Stream states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StreamState {
    Idle,
    Open,
    HalfClosedLocal,
    HalfClosedRemote,
    Closed,
}

/// Flow control manager
#[derive(Debug)]
pub struct FlowController {
    connection_window: i32,
    initial_stream_window: i32,
    stream_windows: std::collections::HashMap<u32, i32>,
}

impl FlowController {
    /// I'm creating a flow controller with default window sizes
    pub fn new(initial_window: i32) -> Self {
        Self {
            connection_window: initial_window,
            initial_stream_window: initial_window,
            stream_windows: std::collections::HashMap::new(),
        }
    }

    /// Update connection window
    pub fn update_connection_window(&mut self, delta: i32) -> Result<(), Error> {
        self.connection_window = self.connection_window
            .checked_add(delta)
            .ok_or(Error::FlowControlError)?;
        
        if self.connection_window < 0 || self.connection_window > i32::MAX / 2 {
            return Err(Error::FlowControlError);
        }

        Ok(())
    }

    /// Update stream window
    pub fn update_stream_window(&mut self, stream_id: u32, delta: i32) -> Result<(), Error> {
        let window = self.stream_windows
            .entry(stream_id)
            .or_insert(self.initial_stream_window);
        
        *window = window.checked_add(delta)
            .ok_or(Error::FlowControlError)?;
        
        if *window < 0 || *window > i32::MAX / 2 {
            return Err(Error::FlowControlError);
        }

        Ok(())
    }

    /// Check if data can be sent
    pub fn can_send(&self, stream_id: u32, size: usize) -> bool {
        if size as i32 > self.connection_window {
            return false;
        }

        if let Some(&stream_window) = self.stream_windows.get(&stream_id) {
            size as i32 <= stream_window
        } else {
            size as i32 <= self.initial_stream_window
        }
    }

    /// Consume window after sending data
    pub fn consume_window(&mut self, stream_id: u32, size: usize) -> Result<(), Error> {
        let size = size as i32;
        
        self.connection_window -= size;
        
        let stream_window = self.stream_windows
            .entry(stream_id)
            .or_insert(self.initial_stream_window);
        *stream_window -= size;

        Ok(())
    }
}

/// Performance metrics for protocol parsing
#[derive(Debug, Default)]
pub struct ParserMetrics {
    pub requests_parsed: u64,
    pub bytes_processed: u64,
    pub parse_errors: u64,
    pub average_parse_time_ns: u64,
}

impl ParserMetrics {
    /// Update metrics after parsing
    pub fn record_parse(&mut self, bytes: usize, duration_ns: u64, success: bool) {
        self.bytes_processed += bytes as u64;
        
        if success {
            self.requests_parsed += 1;
            // Update rolling average
            self.average_parse_time_ns = 
                (self.average_parse_time_ns * self.requests_parsed + duration_ns) 
                / (self.requests_parsed + 1);
        } else {
            self.parse_errors += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_detection() {
        // HTTP/1.1
        let http1_request = b"GET / HTTP/1.1\r\n";
        let detection = detect_protocol(http1_request).unwrap();
        assert_eq!(detection.protocol, Protocol::Http1);

        // HTTP/2
        let http2_preface = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";
        let detection = detect_protocol(http2_preface).unwrap();
        assert_eq!(detection.protocol, Protocol::Http2);

        // Invalid
        assert!(detect_protocol(b"INVALID").is_none());
    }

    #[test]
    fn test_stream_manager() {
        let mut manager = StreamManager::new(Some(3));
        
        // Create streams
        let stream1 = manager.create_stream().unwrap();
        let stream2 = manager.create_stream().unwrap();
        let stream3 = manager.create_stream().unwrap();
        
        assert_eq!(stream1, 1);
        assert_eq!(stream2, 3);
        assert_eq!(stream3, 5);
        
        // Should fail - max concurrent streams
        assert!(manager.create_stream().is_err());
        
        // Close a stream
        manager.update_stream(stream1, StreamState::Closed);
        manager.cleanup_closed_streams();
        
        // Now we can create another
        assert!(manager.create_stream().is_ok());
    }

    #[test]
    fn test_flow_controller() {
        let mut flow = FlowController::new(65535);
        
        // Check initial state
        assert!(flow.can_send(1, 1000));
        assert!(flow.can_send(1, 65535));
        assert!(!flow.can_send(1, 65536));
        
        // Consume window
        flow.consume_window(1, 1000).unwrap();
        assert!(flow.can_send(1, 64535));
        assert!(!flow.can_send(1, 64536));
        
        // Update window
        flow.update_stream_window(1, 1000).unwrap();
        assert!(flow.can_send(1, 65535));
    }

    #[test]
    fn test_parser_metrics() {
        let mut metrics = ParserMetrics::default();
        
        metrics.record_parse(100, 1000, true);
        assert_eq!(metrics.requests_parsed, 1);
        assert_eq!(metrics.bytes_processed, 100);
        assert_eq!(metrics.average_parse_time_ns, 1000);
        
        metrics.record_parse(200, 2000, true);
        assert_eq!(metrics.requests_parsed, 2);
        assert_eq!(metrics.bytes_processed, 300);
        assert_eq!(metrics.average_parse_time_ns, 1500);
        
        metrics.record_parse(50, 0, false);
        assert_eq!(metrics.parse_errors, 1);
        assert_eq!(metrics.bytes_processed, 350);
    }
}
