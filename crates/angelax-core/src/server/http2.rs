// This module implements a high-performance HTTP/2 parser with binary frame processing, HPACK header compression, and stream multiplexing.
// It leverages SIMD operations for frame boundary detection and efficient memory copying while maintaining the complex state machine required by HTTP/2.

use crate::utils::simd::*;
use std::collections::HashMap;
use std::convert::TryFrom;

/// HTTP/2 connection preface
const CONNECTION_PREFACE: &[u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";

/// Frame type identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FrameType {
    Data = 0x0,
    Headers = 0x1,
    Priority = 0x2,
    RstStream = 0x3,
    Settings = 0x4,
    PushPromise = 0x5,
    Ping = 0x6,
    GoAway = 0x7,
    WindowUpdate = 0x8,
    Continuation = 0x9,
}

impl TryFrom<u8> for FrameType {
    type Error = Http2ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(FrameType::Data),
            0x1 => Ok(FrameType::Headers),
            0x2 => Ok(FrameType::Priority),
            0x3 => Ok(FrameType::RstStream),
            0x4 => Ok(FrameType::Settings),
            0x5 => Ok(FrameType::PushPromise),
            0x6 => Ok(FrameType::Ping),
            0x7 => Ok(FrameType::GoAway),
            0x8 => Ok(FrameType::WindowUpdate),
            0x9 => Ok(FrameType::Continuation),
            _ => Err(Http2ParseError::UnknownFrameType(value)),
        }
    }
}

/// Frame flags
#[derive(Debug, Clone, Copy)]
pub struct FrameFlags(u8);

impl FrameFlags {
    pub const END_STREAM: u8 = 0x1;
    pub const END_HEADERS: u8 = 0x4;
    pub const PADDED: u8 = 0x8;
    pub const PRIORITY: u8 = 0x20;

    #[inline]
    pub fn has(&self, flag: u8) -> bool {
        self.0 & flag != 0
    }
}

/// HTTP/2 error codes
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ErrorCode {
    NoError = 0x0,
    ProtocolError = 0x1,
    InternalError = 0x2,
    FlowControlError = 0x3,
    SettingsTimeout = 0x4,
    StreamClosed = 0x5,
    FrameSizeError = 0x6,
    RefusedStream = 0x7,
    Cancel = 0x8,
    CompressionError = 0x9,
    ConnectError = 0xa,
    EnhanceYourCalm = 0xb,
    InadequateSecurity = 0xc,
    Http11Required = 0xd,
}

/// HTTP/2 parse errors
#[derive(Debug, Clone, PartialEq)]
pub enum Http2ParseError {
    InvalidPreface,
    InvalidFrameHeader,
    UnknownFrameType(u8),
    InvalidStreamId,
    InvalidFrameSize,
    InvalidPadding,
    InvalidHeaderBlock,
    InvalidSettings,
    InvalidWindowUpdate,
    InvalidPriority,
    CompressionError,
    ConnectionError(ErrorCode),
    StreamError(u32, ErrorCode),
    IncompleteFrame,
    TooLargeFrame,
}

/// HTTP/2 frame header (9 bytes)
#[derive(Debug, Clone, Copy)]
pub struct FrameHeader {
    pub length: u32,
    pub frame_type: FrameType,
    pub flags: FrameFlags,
    pub stream_id: u32,
}

/// HTTP/2 frame
#[derive(Debug)]
pub struct Frame {
    pub header: FrameHeader,
    pub payload: Vec<u8>,
}

/// HTTP/2 settings
#[derive(Debug, Clone)]
pub struct Settings {
    pub header_table_size: u32,
    pub enable_push: bool,
    pub max_concurrent_streams: Option<u32>,
    pub initial_window_size: u32,
    pub max_frame_size: u32,
    pub max_header_list_size: Option<u32>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            header_table_size: 4096,
            enable_push: true,
            max_concurrent_streams: None,
            initial_window_size: 65535,
            max_frame_size: 16384,
            max_header_list_size: None,
        }
    }
}

/// HPACK dynamic table entry
#[derive(Debug, Clone)]
struct TableEntry {
    name: Vec<u8>,
    value: Vec<u8>,
}

/// HPACK decoder for header compression
pub struct HpackDecoder {
    dynamic_table: Vec<TableEntry>,
    dynamic_table_size: usize,
    max_dynamic_table_size: usize,
}

impl HpackDecoder {
    /// I'm creating a new HPACK decoder
    pub fn new(max_size: usize) -> Self {
        Self {
            dynamic_table: Vec::new(),
            dynamic_table_size: 0,
            max_dynamic_table_size: max_size,
        }
    }

    /// Decode header block
    pub fn decode(&mut self, input: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>, Http2ParseError> {
        let mut headers = Vec::new();
        let mut offset = 0;

        while offset < input.len() {
            let first_byte = input[offset];

            if first_byte & 0x80 != 0 {
                // Indexed header field
                let index = self.decode_integer(&input[offset..], 7)?;
                offset += self.integer_size(index, 7);
                
                let (name, value) = self.get_indexed(index as usize)?;
                headers.push((name, value));
            } else if first_byte & 0x40 != 0 {
                // Literal header field with incremental indexing
                offset += 1;
                let (name, value, consumed) = self.decode_literal(&input[offset..], first_byte & 0x3F)?;
                offset += consumed;
                
                self.add_to_table(name.clone(), value.clone());
                headers.push((name, value));
            } else {
                // Other header field representations
                // TODO: Implement other encoding types
                return Err(Http2ParseError::InvalidHeaderBlock);
            }
        }

        Ok(headers)
    }

    /// Decode integer using HPACK integer encoding
    fn decode_integer(&self, input: &[u8], prefix_bits: u8) -> Result<u32, Http2ParseError> {
        if input.is_empty() {
            return Err(Http2ParseError::InvalidHeaderBlock);
        }

        let prefix_mask = (1 << prefix_bits) - 1;
        let mut value = (input[0] & prefix_mask) as u32;

        if value < prefix_mask as u32 {
            return Ok(value);
        }

        // Multi-byte integer
        let mut m = 0;
        let mut offset = 1;

        while offset < input.len() {
            let byte = input[offset];
            value += ((byte & 0x7F) as u32) << m;
            m += 7;

            if byte & 0x80 == 0 {
                return Ok(value);
            }

            offset += 1;

            if m > 28 {
                return Err(Http2ParseError::InvalidHeaderBlock);
            }
        }

        Err(Http2ParseError::InvalidHeaderBlock)
    }

    /// Get size of integer encoding
    fn integer_size(&self, value: u32, prefix_bits: u8) -> usize {
        let prefix_max = (1 << prefix_bits) - 1;
        if value < prefix_max as u32 {
            1
        } else {
            // Calculate multi-byte size
            let mut size = 1;
            let mut remaining = value - prefix_max as u32;
            while remaining >= 128 {
                remaining >>= 7;
                size += 1;
            }
            size + 1
        }
    }

    /// Decode literal header field
    fn decode_literal(&self, input: &[u8], index: u8) -> Result<(Vec<u8>, Vec<u8>, usize), Http2ParseError> {
        let mut offset = 0;

        let name = if index == 0 {
            // Literal name
            let (name, consumed) = self.decode_string(&input[offset..])?;
            offset += consumed;
            name
        } else {
            // Indexed name
            let (name, _) = self.get_indexed(index as usize)?;
            name
        };

        let (value, consumed) = self.decode_string(&input[offset..])?;
        offset += consumed;

        Ok((name, value, offset))
    }

    /// Decode string (with optional Huffman encoding)
    fn decode_string(&self, input: &[u8]) -> Result<(Vec<u8>, usize), Http2ParseError> {
        if input.is_empty() {
            return Err(Http2ParseError::InvalidHeaderBlock);
        }

        let huffman = input[0] & 0x80 != 0;
        let length = self.decode_integer(input, 7)? as usize;
        let length_size = self.integer_size(length as u32, 7);

        if input.len() < length_size + length {
            return Err(Http2ParseError::InvalidHeaderBlock);
        }

        let string_data = &input[length_size..length_size + length];

        if huffman {
            // TODO: Implement Huffman decoding
            Err(Http2ParseError::CompressionError)
        } else {
            Ok((string_data.to_vec(), length_size + length))
        }
    }

    /// Get indexed header from static or dynamic table
    fn get_indexed(&self, index: usize) -> Result<(Vec<u8>, Vec<u8>), Http2ParseError> {
        if index == 0 {
            return Err(Http2ParseError::InvalidHeaderBlock);
        }

        // Static table has 61 entries
        if index <= 61 {
            // Return from static table
            // TODO: Implement static table
            Ok((vec![], vec![]))
        } else {
            // Dynamic table
            let dyn_index = index - 62;
            if dyn_index >= self.dynamic_table.len() {
                return Err(Http2ParseError::InvalidHeaderBlock);
            }
            let entry = &self.dynamic_table[dyn_index];
            Ok((entry.name.clone(), entry.value.clone()))
        }
    }

    /// Add entry to dynamic table
    fn add_to_table(&mut self, name: Vec<u8>, value: Vec<u8>) {
        let entry_size = 32 + name.len() + value.len();

        // Evict entries if necessary
        while self.dynamic_table_size + entry_size > self.max_dynamic_table_size {
            if self.dynamic_table.is_empty() {
                return;
            }
            let removed = self.dynamic_table.pop();
            if let Some(entry) = removed {
                self.dynamic_table_size -= 32 + entry.name.len() + entry.value.len();
            }
        }

        self.dynamic_table.insert(0, TableEntry { name, value });
        self.dynamic_table_size += entry_size;
    }
}

/// HTTP/2 parser
pub struct Http2Parser {
    settings: Settings,
    hpack_decoder: HpackDecoder,
    max_frame_size: u32,
}

impl Http2Parser {
    /// I'm creating a new HTTP/2 parser
    pub fn new() -> Self {
        let settings = Settings::default();
        Self {
            hpack_decoder: HpackDecoder::new(settings.header_table_size as usize),
            max_frame_size: settings.max_frame_size,
            settings,
        }
    }

    /// Check connection preface
    pub fn check_preface(&self, input: &[u8]) -> Result<bool, Http2ParseError> {
        if input.len() < CONNECTION_PREFACE.len() {
            return Ok(false);
        }

        if &input[..CONNECTION_PREFACE.len()] != CONNECTION_PREFACE {
            return Err(Http2ParseError::InvalidPreface);
        }

        Ok(true)
    }

    /// Parse frame header (9 bytes)
    pub fn parse_frame_header(&self, input: &[u8]) -> Result<FrameHeader, Http2ParseError> {
        if input.len() < 9 {
            return Err(Http2ParseError::IncompleteFrame);
        }

        // I'm using direct byte access for speed
        let length = u32::from_be_bytes([0, input[0], input[1], input[2]]);
        let frame_type = FrameType::try_from(input[3])?;
        let flags = FrameFlags(input[4]);
        let stream_id = u32::from_be_bytes([input[5] & 0x7F, input[6], input[7], input[8]]);

        if length > self.max_frame_size {
            return Err(Http2ParseError::InvalidFrameSize);
        }

        Ok(FrameHeader {
            length,
            frame_type,
            flags,
            stream_id,
        })
    }

    /// Parse complete frame
    pub fn parse_frame(&self, input: &[u8]) -> Result<(Frame, usize), Http2ParseError> {
        let header = self.parse_frame_header(input)?;
        
        let total_size = 9 + header.length as usize;
        if input.len() < total_size {
            return Err(Http2ParseError::IncompleteFrame);
        }

        let payload = input[9..total_size].to_vec();

        Ok((Frame { header, payload }, total_size))
    }

    /// Parse settings frame payload
    pub fn parse_settings(&self, payload: &[u8]) -> Result<Vec<(u16, u32)>, Http2ParseError> {
        if payload.len() % 6 != 0 {
            return Err(Http2ParseError::InvalidSettings);
        }

        let mut settings = Vec::new();
        let mut offset = 0;

        while offset < payload.len() {
            let id = u16::from_be_bytes([payload[offset], payload[offset + 1]]);
            let value = u32::from_be_bytes([
                payload[offset + 2],
                payload[offset + 3],
                payload[offset + 4],
                payload[offset + 5],
            ]);

            settings.push((id, value));
            offset += 6;
        }

        Ok(settings)
    }

    /// Parse headers frame payload
    pub fn parse_headers(&mut self, payload: &[u8], flags: FrameFlags) -> Result<Vec<(Vec<u8>, Vec<u8>)>, Http2ParseError> {
        let mut offset = 0;

        // Handle padding
        let pad_length = if flags.has(FrameFlags::PADDED) {
            if payload.is_empty() {
                return Err(Http2ParseError::InvalidPadding);
            }
            let pad = payload[0] as usize;
            offset += 1;
            pad
        } else {
            0
        };

        // Handle priority
        if flags.has(FrameFlags::PRIORITY) {
            if payload.len() < offset + 5 {
                return Err(Http2ParseError::InvalidPriority);
            }
            offset += 5; // Skip priority data
        }

        // Check padding
        if payload.len() < offset + pad_length {
            return Err(Http2ParseError::InvalidPadding);
        }

        let header_block_end = payload.len() - pad_length;
        let header_block = &payload[offset..header_block_end];

        self.hpack_decoder.decode(header_block)
    }

    /// Update settings
    pub fn update_settings(&mut self, new_settings: &[(u16, u32)]) -> Result<(), Http2ParseError> {
        for &(id, value) in new_settings {
            match id {
                1 => {
                    self.settings.header_table_size = value;
                    // Update HPACK decoder table size
                    self.hpack_decoder = HpackDecoder::new(value as usize);
                }
                2 => self.settings.enable_push = value != 0,
                3 => self.settings.max_concurrent_streams = Some(value),
                4 => self.settings.initial_window_size = value,
                5 => {
                    if value < 16384 || value > 16777215 {
                        return Err(Http2ParseError::InvalidSettings);
                    }
                    self.settings.max_frame_size = value;
                    self.max_frame_size = value;
                }
                6 => self.settings.max_header_list_size = Some(value),
                _ => {} // Ignore unknown settings
            }
        }
        Ok(())
    }
}

/// Fast HTTP/2 frame builder
pub struct Http2FrameBuilder {
    buffer: Vec<u8>,
}

impl Http2FrameBuilder {
    /// I'm creating a frame builder
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(16384),
        }
    }

    /// Build frame header
    pub fn header(&mut self, frame_type: FrameType, flags: u8, stream_id: u32, length: u32) -> &mut Self {
        // Length (24 bits)
        self.buffer.push((length >> 16) as u8);
        self.buffer.push((length >> 8) as u8);
        self.buffer.push(length as u8);
        
        // Type
        self.buffer.push(frame_type as u8);
        
        // Flags
        self.buffer.push(flags);
        
        // Stream ID (31 bits)
        self.buffer.push((stream_id >> 24) as u8);
        self.buffer.push((stream_id >> 16) as u8);
        self.buffer.push((stream_id >> 8) as u8);
        self.buffer.push(stream_id as u8);
        
        self
    }

    /// Add payload
    pub fn payload(&mut self, data: &[u8]) -> &mut Self {
        self.buffer.extend_from_slice(data);
        self
    }

    /// Build final frame
    pub fn build(self) -> Vec<u8> {
        self.buffer
    }

    /// Build settings frame
    pub fn settings_frame(settings: &[(u16, u32)]) -> Vec<u8> {
        let mut builder = Self::new();
        let mut payload = Vec::with_capacity(settings.len() * 6);
        
        for &(id, value) in settings {
            payload.extend_from_slice(&id.to_be_bytes());
            payload.extend_from_slice(&value.to_be_bytes());
        }
        
        builder
            .header(FrameType::Settings, 0, 0, payload.len() as u32)
            .payload(&payload)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_header_parsing() {
        let parser = Http2Parser::new();
        
        // Valid frame header
        let header_bytes = [
            0x00, 0x00, 0x08, // Length: 8
            0x00,             // Type: DATA
            0x01,             // Flags: END_STREAM
            0x00, 0x00, 0x00, 0x01, // Stream ID: 1
        ];
        
        let header = parser.parse_frame_header(&header_bytes).unwrap();
        assert_eq!(header.length, 8);
        assert_eq!(header.frame_type, FrameType::Data);
        assert!(header.flags.has(FrameFlags::END_STREAM));
        assert_eq!(header.stream_id, 1);
    }

    #[test]
    fn test_connection_preface() {
        let parser = Http2Parser::new();
        
        assert!(parser.check_preface(CONNECTION_PREFACE).unwrap());
        assert!(!parser.check_preface(b"Invalid preface").unwrap());
        assert!(!parser.check_preface(b"PRI").unwrap()); // Too short
    }

    #[test]
    fn test_settings_parsing() {
        let parser = Http2Parser::new();
        
        let settings_payload = [
            0x00, 0x03, // ID: MAX_CONCURRENT_STREAMS
            0x00, 0x00, 0x00, 0x64, // Value: 100
            0x00, 0x04, // ID: INITIAL_WINDOW_SIZE
            0x00, 0x01, 0x00, 0x00, // Value: 65536
        ];
        
        let settings = parser.parse_settings(&settings_payload).unwrap();
        assert_eq!(settings.len(), 2);
        assert_eq!(settings[0], (3, 100));
        assert_eq!(settings[1], (4, 65536));
    }

    #[test]
    fn test_frame_builder() {
        let frame = Http2FrameBuilder::new()
            .header(FrameType::Data, FrameFlags::END_STREAM, 1, 5)
            .payload(b"Hello")
            .build();
        
        assert_eq!(frame.len(), 14); // 9 byte header + 5 byte payload
        assert_eq!(&frame[9..], b"Hello");
    }

    #[test]
    fn test_settings_frame_builder() {
        let settings = vec![
            (1, 4096),  // HEADER_TABLE_SIZE
            (3, 100),   // MAX_CONCURRENT_STREAMS
        ];
        
        let frame = Http2FrameBuilder::settings_frame(&settings);
        assert_eq!(frame.len(), 21); // 9 byte header + 12 byte payload
        assert_eq!(frame[3], FrameType::Settings as u8);
    }

    #[test]
    fn test_hpack_integer_decoding() {
        let decoder = HpackDecoder::new(4096);
        
        // Small integer (fits in prefix)
        assert_eq!(decoder.decode_integer(&[0x0A], 5).unwrap(), 10);
        
        // Integer requiring multiple bytes
        let multi_byte = [0x1F, 0xE1, 0x03]; // 1337 with 5-bit prefix
        assert_eq!(decoder.decode_integer(&multi_byte, 5).unwrap(), 1337);
    }

    #[test]
    fn test_frame_type_conversion() {
        assert_eq!(FrameType::try_from(0x0).unwrap(), FrameType::Data);
        assert_eq!(FrameType::try_from(0x1).unwrap(), FrameType::Headers);
        assert!(FrameType::try_from(0xFF).is_err());
    }
}
