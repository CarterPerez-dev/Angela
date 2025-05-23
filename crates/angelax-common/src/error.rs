// This module defines the core error types used throughout the Angelax framework.
// It provides a unified error handling approach with detailed error information while maintaining zero-cost abstractions for the common path.

use std::fmt;
use std::error::Error as StdError;
use std::io;

/// Core error type for the Angelax framework
#[derive(Debug)]
pub enum Error {
    /// I/O errors
    Io(io::Error),
    
    /// Parse errors with details
    ParseError(String),
    
    /// Invalid protocol
    InvalidProtocol,
    
    /// Connection closed
    ConnectionClosed,
    
    /// Too many streams
    TooManyStreams,
    
    /// Flow control error
    FlowControlError,
    
    /// Request timeout
    Timeout,
    
    /// Request too large
    RequestTooLarge,
    
    /// Invalid header
    InvalidHeader(String),
    
    /// Invalid state
    InvalidState(String),
    
    /// Not implemented
    NotImplemented(&'static str),
    
    /// TLS error
    TlsError(String),
    
    /// Configuration error
    ConfigError(String),
    
    /// Resource exhausted
    ResourceExhausted(String),
    
    /// Internal error
    Internal(String),
    
    /// Custom error with error code
    Custom {
        code: ErrorCode,
        message: String,
    },
}

/// Error codes for structured error handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ErrorCode {
    // Client errors (4xx equivalent)
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    RequestTimeout = 408,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    TooManyRequests = 429,
    
    // Server errors (5xx equivalent)
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    
    // Custom Angelax error codes (6xx)
    ParseError = 600,
    ProtocolError = 601,
    TlsError = 602,
    FlowControlError = 603,
    StreamError = 604,
    ConnectionError = 605,
    ConfigurationError = 606,
    ResourceExhausted = 607,
}

impl Error {
    /// Get the error code
    pub fn code(&self) -> ErrorCode {
        match self {
            Error::ParseError(_) => ErrorCode::ParseError,
            Error::InvalidProtocol => ErrorCode::ProtocolError,
            Error::ConnectionClosed => ErrorCode::ConnectionError,
            Error::TooManyStreams => ErrorCode::ResourceExhausted,
            Error::FlowControlError => ErrorCode::FlowControlError,
            Error::Timeout => ErrorCode::RequestTimeout,
            Error::RequestTooLarge => ErrorCode::PayloadTooLarge,
            Error::InvalidHeader(_) => ErrorCode::BadRequest,
            Error::InvalidState(_) => ErrorCode::InternalServerError,
            Error::NotImplemented(_) => ErrorCode::NotImplemented,
            Error::TlsError(_) => ErrorCode::TlsError,
            Error::ConfigError(_) => ErrorCode::ConfigurationError,
            Error::ResourceExhausted(_) => ErrorCode::ResourceExhausted,
            Error::Internal(_) => ErrorCode::InternalServerError,
            Error::Custom { code, .. } => *code,
            Error::Io(_) => ErrorCode::InternalServerError,
        }
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self.code(),
            ErrorCode::ServiceUnavailable
                | ErrorCode::GatewayTimeout
                | ErrorCode::TooManyRequests
                | ErrorCode::ResourceExhausted
        )
    }

    /// Check if error is client error
    pub fn is_client_error(&self) -> bool {
        let code = self.code() as u16;
        code >= 400 && code < 500
    }

    /// Check if error is server error
    pub fn is_server_error(&self) -> bool {
        let code = self.code() as u16;
        code >= 500 && code < 600
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Error::InvalidProtocol => write!(f, "Invalid protocol"),
            Error::ConnectionClosed => write!(f, "Connection closed"),
            Error::TooManyStreams => write!(f, "Too many concurrent streams"),
            Error::FlowControlError => write!(f, "Flow control error"),
            Error::Timeout => write!(f, "Request timeout"),
            Error::RequestTooLarge => write!(f, "Request too large"),
            Error::InvalidHeader(msg) => write!(f, "Invalid header: {}", msg),
            Error::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            Error::NotImplemented(feature) => write!(f, "Not implemented: {}", feature),
            Error::TlsError(msg) => write!(f, "TLS error: {}", msg),
            Error::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            Error::ResourceExhausted(msg) => write!(f, "Resource exhausted: {}", msg),
            Error::Internal(msg) => write!(f, "Internal error: {}", msg),
            Error::Custom { code, message } => write!(f, "Error {}: {}", *code as u16, message),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::Io(e) => e,
            _ => io::Error::new(io::ErrorKind::Other, err.to_string()),
        }
    }
}

/// Result type alias for Angelax operations
pub type Result<T> = std::result::Result<T, Error>;

/// Extension trait for adding context to errors
pub trait ErrorContext<T> {
    /// Add context to an error
    fn context(self, msg: &str) -> Result<T>;
    
    /// Add context with a closure (lazy evaluation)
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String;
}

impl<T, E> ErrorContext<T> for std::result::Result<T, E>
where
    E: Into<Error>,
{
    fn context(self, msg: &str) -> Result<T> {
        self.map_err(|e| {
            let base_error = e.into();
            Error::Internal(format!("{}: {}", msg, base_error))
        })
    }

    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            let base_error = e.into();
            Error::Internal(format!("{}: {}", f(), base_error))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        assert_eq!(Error::Timeout.code(), ErrorCode::RequestTimeout);
        assert_eq!(Error::RequestTooLarge.code(), ErrorCode::PayloadTooLarge);
        assert_eq!(Error::NotImplemented("test").code(), ErrorCode::NotImplemented);
    }

    #[test]
    fn test_error_classification() {
        assert!(Error::InvalidHeader("test".to_string()).is_client_error());
        assert!(Error::Internal("test".to_string()).is_server_error());
        assert!(!Error::InvalidHeader("test".to_string()).is_server_error());
    }

    #[test]
    fn test_retryable_errors() {
        assert!(Error::Custom {
            code: ErrorCode::ServiceUnavailable,
            message: "Service down".to_string(),
        }
        .is_retryable());
        
        assert!(!Error::InvalidHeader("bad".to_string()).is_retryable());
    }

    #[test]
    fn test_error_context() {
        let result: std::result::Result<(), io::Error> = Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File not found",
        ));
        
        let with_context = result.context("Failed to open config file");
        assert!(with_context.is_err());
        
        let err = with_context.unwrap_err();
        assert!(err.to_string().contains("Failed to open config file"));
    }

    #[test]
    fn test_error_display() {
        let err = Error::ParseError("Invalid JSON".to_string());
        assert_eq!(err.to_string(), "Parse error: Invalid JSON");
        
        let err = Error::Custom {
            code: ErrorCode::BadRequest,
            message: "Missing header".to_string(),
        };
        assert_eq!(err.to_string(), "Error 400: Missing header");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::BrokenPipe, "Pipe broken");
        let err: Error = io_err.into();
        
        match err {
            Error::Io(_) => {},
            _ => panic!("Expected Io variant"),
        }
        
        // Convert back
        let _io_err: io::Error = err.into();
    }
}
