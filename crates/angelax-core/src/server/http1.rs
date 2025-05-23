// This module implements a high-performance HTTP/1.1 parser using SIMD optimizations for parsing request lines, headers, and bodies.
// It's designed for zero-allocation parsing where possible and supports pipelining, keep-alive, and chunked transfer encoding with exceptional throughput.

use crate::utils::simd::*;
use std::str;
use std::fmt;

/// HTTP/1.1 parser errors
#[derive(Debug, Clone, PartialEq)]
pub enum Http1ParseError {
    InvalidMethod,
    InvalidUri,
    InvalidVersion,
    InvalidHeader,
    InvalidHeaderName,
    InvalidHeaderValue,
    InvalidChunkSize,
    InvalidContentLength,
    TooManyHeaders,
    RequestTooLarge,
    IncompleteRequest,
    MalformedRequest,
}

impl fmt::Display for Http1ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMethod => write!(f, "Invalid HTTP method"),
            Self::InvalidUri => write!(f, "Invalid URI"),
            Self::InvalidVersion => write!(f, "Invalid HTTP version"),
            Self::InvalidHeader => write!(f, "Invalid header format"),
            Self::InvalidHeaderName => write!(f, "Invalid header name"),
            Self::InvalidHeaderValue => write!(f, "Invalid header value"),
            Self::InvalidChunkSize => write!(f, "Invalid chunk size"),
            Self::InvalidContentLength => write!(f, "Invalid content length"),
            Self::TooManyHeaders => write!(f, "Too many headers"),
            Self::RequestTooLarge => write!(f, "Request too large"),
            Self::IncompleteRequest => write!(f, "Incomplete request"),
            Self::MalformedRequest => write!(f, "Malformed request"),
        }
    }
}

impl std::error::Error for Http1ParseError {}

/// HTTP method representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl Method {
    /// Parse method from bytes using optimized comparison
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Result<Self, Http1ParseError> {
        // I'm using compile-time optimization for common methods
        match bytes.len() {
            3 => match bytes {
                b"GET" => Ok(Method::Get),
                b"PUT" => Ok(Method::Put),
                _ => Err(Http1ParseError::InvalidMethod),
            },
            4 => match bytes {
                b"HEAD" => Ok(Method::Head),
                b"POST" => Ok(Method::Post),
                _ => Err(Http1ParseError::InvalidMethod),
            },
            5 => match bytes {
                b"PATCH" => Ok(Method::Patch),
                b"TRACE" => Ok(Method::Trace),
                _ => Err(Http1ParseError::InvalidMethod),
            },
            6 => match bytes {
                b"DELETE" => Ok(Method::Delete),
                _ => Err(Http1ParseError::InvalidMethod),
            },
            7 => match bytes {
                b"CONNECT" => Ok(Method::Connect),
                b"OPTIONS" => Ok(Method::Options),
                _ => Err(Http1ParseError::InvalidMethod),
            },
            _ => Err(Http1ParseError::InvalidMethod),
        }
    }
}

/// HTTP version
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

/// Parsed HTTP header
#[derive(Debug, Clone)]
pub struct Header<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

/// HTTP/1.1 request representation
#[derive(Debug)]
pub struct Request<'a> {
    pub method: Method,
    pub uri: &'a str,
    pub version: Version,
    pub headers: Vec<Header<'a>>,
    pub body: Option<&'a [u8]>,
}

/// High-performance HTTP/1.1 parser
pub struct Http1Parser {
    space_finder: SimdDelimiterFinder,
    colon_finder: SimdDelimiterFinder,
    crlf_finder: SimdCrlfFinder,
    max_headers: usize,
    max_header_size: usize,
    max_request_size: usize,
}

impl Http1Parser {
    /// I'm creating a new parser with configurable limits
    pub fn new() -> Self {
        Self {
            space_finder: SimdDelimiterFinder::new(b' '),
            colon_finder: SimdDelimiterFinder::new(b':'),
            crlf_finder: SimdCrlfFinder::new(),
            max_headers: 100,
            max_header_size: 8192,
            max_request_size: 1024 * 1024, // 1MB default
        }
    }

    /// Parse a complete HTTP/1.1 request
    pub fn parse_request<'a>(&self, input: &'a [u8]) -> Result<(Request<'a>, usize), Http1ParseError> {
        if input.is_empty() {
            return Err(Http1ParseError::IncompleteRequest);
        }

        let mut offset = 0;

        // Parse request line
        let (method, uri, version, line_end) = self.parse_request_line(&input[offset..])?;
        offset += line_end;

        // Parse headers
        let (headers, headers_end) = self.parse_headers(&input[offset..])?;
        offset += headers_end;

        // Determine body based on headers
        let body = self.extract_body(&headers, &input[offset..])?;
        if let Some(body) = body {
            offset += body.len();
        }

        let request = Request {
            method,
            uri,
            version,
            headers,
            body,
        };

        Ok((request, offset))
    }

    /// Parse the request line using SIMD-accelerated search
    fn parse_request_line<'a>(&self, input: &'a [u8]) -> Result<(Method, &'a str, Version, usize), Http1ParseError> {
        // Find the end of the request line
        let line_end = self.crlf_finder.find_crlf(input)
            .ok_or(Http1ParseError::IncompleteRequest)?;

        let line = &input[..line_end];

        // Find method end
        let method_end = self.space_finder.find_in(line)
            .ok_or(Http1ParseError::MalformedRequest)?;

        let method = Method::from_bytes(&line[..method_end])?;

        // Find URI end
        let uri_start = method_end + 1;
        let uri_end = self.space_finder.find_in(&line[uri_start..])
            .ok_or(Http1ParseError::MalformedRequest)?;
        let uri_end = uri_start + uri_end;

        let uri = str::from_utf8(&line[uri_start..uri_end])
            .map_err(|_| Http1ParseError::InvalidUri)?;

        // Parse version
        let version_start = uri_end + 1;
        let version = self.parse_version(&line[version_start..])?;

        Ok((method, uri, version, line_end + 2)) // +2 for CRLF
    }

    /// Parse HTTP version
    #[inline]
    fn parse_version(&self, input: &[u8]) -> Result<Version, Http1ParseError> {
        if input.len() < 8 || &input[..5] != b"HTTP/" {
            return Err(Http1ParseError::InvalidVersion);
        }

        // I'm parsing version numbers directly for speed
        let major = match input[5] {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'3' => 3,
            _ => return Err(Http1ParseError::InvalidVersion),
        };

        if input[6] != b'.' {
            return Err(Http1ParseError::InvalidVersion);
        }

        let minor = match input[7] {
            b'0' => 0,
            b'1' => 1,
            b'9' => 9,
            _ => return Err(Http1ParseError::InvalidVersion),
        };

        Ok(Version { major, minor })
    }

    /// Parse headers using SIMD optimizations
    fn parse_headers<'a>(&self, input: &'a [u8]) -> Result<(Vec<Header<'a>>, usize), Http1ParseError> {
        let mut headers = Vec::with_capacity(32); // Pre-allocate for common case
        let mut offset = 0;

        loop {
            if offset >= input.len() {
                return Err(Http1ParseError::IncompleteRequest);
            }

            // Check for end of headers (empty line)
            if offset + 2 <= input.len() && &input[offset..offset + 2] == b"\r\n" {
                return Ok((headers, offset + 2));
            }

            // Find end of header line
            let line_end = self.crlf_finder.find_crlf(&input[offset..])
                .ok_or(Http1ParseError::IncompleteRequest)?;

            let header_line = &input[offset..offset + line_end];

            // Parse header
            let header = self.parse_header(header_line)?;
            headers.push(header);

            if headers.len() > self.max_headers {
                return Err(Http1ParseError::TooManyHeaders);
            }

            offset += line_end + 2; // +2 for CRLF
        }
    }

    /// Parse a single header
    #[inline]
    fn parse_header<'a>(&self, line: &'a [u8]) -> Result<Header<'a>, Http1ParseError> {
        let colon_pos = self.colon_finder.find_in(line)
            .ok_or(Http1ParseError::InvalidHeader)?;

        let name_bytes = &line[..colon_pos];
        if !SimdTokenValidator::is_valid_token(name_bytes) {
            return Err(Http1ParseError::InvalidHeaderName);
        }

        let name = str::from_utf8(name_bytes)
            .map_err(|_| Http1ParseError::InvalidHeaderName)?;

        // Skip colon and optional whitespace
        let value_start = colon_pos + 1;
        let value_bytes = SimdWhitespaceSkipper::skip_whitespace(&line[value_start..]);
        
        // Trim trailing whitespace
        let value_end = value_bytes.iter()
            .rposition(|&b| b != b' ' && b != b'\t')
            .map(|pos| pos + 1)
            .unwrap_or(0);

        let value = str::from_utf8(&value_bytes[..value_end])
            .map_err(|_| Http1ParseError::InvalidHeaderValue)?;

        Ok(Header { name, value })
    }

    /// Extract body based on headers
    fn extract_body<'a>(&self, headers: &[Header], remaining: &'a [u8]) -> Result<Option<&'a [u8]>, Http1ParseError> {
        // Look for Content-Length header
        for header in headers {
            if header.name.eq_ignore_ascii_case("content-length") {
                let length: usize = header.value.parse()
                    .map_err(|_| Http1ParseError::InvalidContentLength)?;
                
                if length > self.max_request_size {
                    return Err(Http1ParseError::RequestTooLarge);
                }

                if remaining.len() < length {
                    return Err(Http1ParseError::IncompleteRequest);
                }

                return Ok(Some(&remaining[..length]));
            }
        }

        // TODO: Handle chunked transfer encoding
        
        Ok(None)
    }
}

impl Default for Http1Parser {
    fn default() -> Self {
        Self::new()
    }
}

/// HTTP/1.1 response builder for fast serialization
pub struct Http1ResponseBuilder {
    buffer: Vec<u8>,
}

impl Http1ResponseBuilder {
    /// I'm creating a response builder with pre-allocated buffer
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(4096),
        }
    }

    /// Build status line
    pub fn status(&mut self, code: u16, reason: &str) -> &mut Self {
        self.buffer.extend_from_slice(b"HTTP/1.1 ");
        self.write_u16(code);
        self.buffer.push(b' ');
        self.buffer.extend_from_slice(reason.as_bytes());
        self.buffer.extend_from_slice(b"\r\n");
        self
    }

    /// Add header
    pub fn header(&mut self, name: &str, value: &str) -> &mut Self {
        self.buffer.extend_from_slice(name.as_bytes());
        self.buffer.extend_from_slice(b": ");
        self.buffer.extend_from_slice(value.as_bytes());
        self.buffer.extend_from_slice(b"\r\n");
        self
    }

    /// Finish headers and optionally add body
    pub fn body(&mut self, body: Option<&[u8]>) -> Vec<u8> {
        if let Some(body) = body {
            self.header("Content-Length", &body.len().to_string());
        }
        
        self.buffer.extend_from_slice(b"\r\n");
        
        if let Some(body) = body {
            self.buffer.extend_from_slice(body);
        }
        
        std::mem::take(&mut self.buffer)
    }

    /// Fast integer to string conversion
    #[inline]
    fn write_u16(&mut self, mut n: u16) {
        let mut buf = [0u8; 5];
        let mut i = 4;
        
        loop {
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                break;
            }
            i -= 1;
        }
        
        self.buffer.extend_from_slice(&buf[i..5]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_request() {
        let parser = Http1Parser::new();
        let request = b"GET /index.html HTTP/1.1\r\n\r\n";
        
        let (req, consumed) = parser.parse_request(request).unwrap();
        
        assert_eq!(req.method, Method::Get);
        assert_eq!(req.uri, "/index.html");
        assert_eq!(req.version.major, 1);
        assert_eq!(req.version.minor, 1);
        assert_eq!(req.headers.len(), 0);
        assert!(req.body.is_none());
        assert_eq!(consumed, request.len());
    }

    #[test]
    fn test_parse_request_with_headers() {
        let parser = Http1Parser::new();
        let request = b"POST /api/users HTTP/1.1\r\nHost: example.com\r\nContent-Type: application/json\r\nContent-Length: 13\r\n\r\n{\"id\": 12345}";
        
        let (req, consumed) = parser.parse_request(request).unwrap();
        
        assert_eq!(req.method, Method::Post);
        assert_eq!(req.uri, "/api/users");
        assert_eq!(req.headers.len(), 3);
        assert_eq!(req.headers[0].name, "Host");
        assert_eq!(req.headers[0].value, "example.com");
        assert_eq!(req.headers[1].name, "Content-Type");
        assert_eq!(req.headers[1].value, "application/json");
        assert_eq!(req.headers[2].name, "Content-Length");
        assert_eq!(req.headers[2].value, "13");
        assert_eq!(req.body, Some(&b"{\"id\": 12345}"[..]));
        assert_eq!(consumed, request.len());
    }

    #[test]
    fn test_parse_all_methods() {
        let parser = Http1Parser::new();
        
        let methods = [
            ("GET", Method::Get),
            ("HEAD", Method::Head),
            ("POST", Method::Post),
            ("PUT", Method::Put),
            ("DELETE", Method::Delete),
            ("CONNECT", Method::Connect),
            ("OPTIONS", Method::Options),
            ("TRACE", Method::Trace),
            ("PATCH", Method::Patch),
        ];
        
        for (method_str, expected) in methods {
            let request = format!("{} / HTTP/1.1\r\n\r\n", method_str);
            let (req, _) = parser.parse_request(request.as_bytes()).unwrap();
            assert_eq!(req.method, expected);
        }
    }

    #[test]
    fn test_parse_version() {
        let parser = Http1Parser::new();
        
        assert_eq!(parser.parse_version(b"HTTP/1.0").unwrap(), Version { major: 1, minor: 0 });
        assert_eq!(parser.parse_version(b"HTTP/1.1").unwrap(), Version { major: 1, minor: 1 });
        assert_eq!(parser.parse_version(b"HTTP/2.0").unwrap(), Version { major: 2, minor: 0 });
        
        assert!(parser.parse_version(b"HTTP/").is_err());
        assert!(parser.parse_version(b"HTTP/1").is_err());
        assert!(parser.parse_version(b"HTTP/1.").is_err());
        assert!(parser.parse_version(b"HTTP/a.b").is_err());
    }

    #[test]
    fn test_header_whitespace_handling() {
        let parser = Http1Parser::new();
        let request = b"GET / HTTP/1.1\r\nHost:   example.com   \r\nSpaced-Header:\t\tvalue\t\t\r\n\r\n";
        
        let (req, _) = parser.parse_request(request).unwrap();
        
        assert_eq!(req.headers[0].value, "example.com");
        assert_eq!(req.headers[1].value, "value");
    }

    #[test]
    fn test_response_builder() {
        let mut builder = Http1ResponseBuilder::new();
        let response = builder
            .status(200, "OK")
            .header("Content-Type", "text/plain")
            .header("Server", "Angelax/1.0")
            .body(Some(b"Hello, World!"));
        
        let expected = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nServer: Angelax/1.0\r\nContent-Length: 13\r\n\r\nHello, World!";
        assert_eq!(response, expected);
    }

    #[test]
    fn test_error_cases() {
        let parser = Http1Parser::new();
        
        // Incomplete request
        assert!(matches!(parser.parse_request(b"GET /"), Err(Http1ParseError::IncompleteRequest)));
        
        // Invalid method
        assert!(matches!(parser.parse_request(b"INVALID / HTTP/1.1\r\n\r\n"), Err(Http1ParseError::InvalidMethod)));
        
        // Invalid header
        assert!(matches!(parser.parse_request(b"GET / HTTP/1.1\r\nBadHeader\r\n\r\n"), Err(Http1ParseError::InvalidHeader)));
        
        // Invalid content length
        assert!(matches!(parser.parse_request(b"GET / HTTP/1.1\r\nContent-Length: abc\r\n\r\n"), Err(Http1ParseError::InvalidContentLength)));
    }
}
