// This example demonstrates the high-performance HTTP/1.1 and HTTP/2 parsers in action.
// It shows how to parse requests, handle different protocols, and leverage SIMD optimizations for exceptional throughput.

use angelax_core::{
    server::{
        http1::{Http1Parser, Http1ResponseBuilder},
        http2::{Http2Parser, Http2FrameBuilder, FrameType},
        Protocol, detect_protocol,
    },
    utils::pool::BufferPool,
};
use std::time::Instant;

fn main() {
    println!("Angelax HTTP Parser Demo\n");
    
    demo_http1_parsing();
    println!("\n---\n");
    demo_http2_parsing();
    println!("\n---\n");
    demo_protocol_detection();
    println!("\n---\n");
    demo_performance();
}

fn demo_http1_parsing() {
    println!("=== HTTP/1.1 Parsing Demo ===");
    
    let parser = Http1Parser::new();
    
    // Example 1: Simple GET request
    let simple_request = b"GET /api/users HTTP/1.1\r\nHost: example.com\r\n\r\n";
    
    match parser.parse_request(simple_request) {
        Ok((request, consumed)) => {
            println!("✓ Parsed simple GET request:");
            println!("  Method: {:?}", request.method);
            println!("  URI: {}", request.uri);
            println!("  Version: HTTP/{}.{}", request.version.major, request.version.minor);
            println!("  Headers: {} total", request.headers.len());
            for header in &request.headers {
                println!("    {}: {}", header.name, header.value);
            }
            println!("  Consumed: {} bytes", consumed);
        }
        Err(e) => eprintln!("✗ Failed to parse: {:?}", e),
    }
    
    // Example 2: POST request with body
    let post_request = b"POST /api/users HTTP/1.1\r\n\
Host: api.example.com\r\n\
Content-Type: application/json\r\n\
Content-Length: 27\r\n\
\r\n\
{\"name\":\"Alice\",\"age\":30}";
    
    println!("\n✓ Parsing POST request with body:");
    match parser.parse_request(post_request) {
        Ok((request, consumed)) => {
            println!("  Method: {:?}", request.method);
            println!("  Body: {:?}", 
                request.body.map(|b| String::from_utf8_lossy(b).to_string())
            );
            println!("  Total consumed: {} bytes", consumed);
        }
        Err(e) => eprintln!("✗ Failed to parse: {:?}", e),
    }
    
    // Example 3: Building responses
    println!("\n✓ Building HTTP/1.1 responses:");
    
    let mut builder = Http1ResponseBuilder::new();
    let response = builder
        .status(200, "OK")
        .header("Content-Type", "application/json")
        .header("X-Request-ID", "12345")
        .body(Some(b"{\"status\":\"success\"}"));
    
    println!("  Response ({} bytes):", response.len());
    println!("{}", String::from_utf8_lossy(&response[..100.min(response.len())]));
}

fn demo_http2_parsing() {
    println!("=== HTTP/2 Parsing Demo ===");
    
    let mut parser = Http2Parser::new();
    
    // Example 1: Parse frame header
    let frame_header = [
        0x00, 0x00, 0x08, // Length: 8
        0x00,             // Type: DATA
        0x01,             // Flags: END_STREAM
        0x00, 0x00, 0x00, 0x01, // Stream ID: 1
    ];
    
    match parser.parse_frame_header(&frame_header) {
        Ok(header) => {
            println!("✓ Parsed HTTP/2 frame header:");
            println!("  Type: {:?}", header.frame_type);
            println!("  Length: {} bytes", header.length);
            println!("  Stream ID: {}", header.stream_id);
            println!("  Flags: 0x{:02x}", header.flags.0);
        }
        Err(e) => eprintln!("✗ Failed to parse frame header: {:?}", e),
    }
    
    // Example 2: Build settings frame
    println!("\n✓ Building HTTP/2 SETTINGS frame:");
    let settings = vec![
        (1, 4096),  // HEADER_TABLE_SIZE
        (3, 100),   // MAX_CONCURRENT_STREAMS
        (4, 65536), // INITIAL_WINDOW_SIZE
    ];
    
    let settings_frame = Http2FrameBuilder::settings_frame(&settings);
    println!("  Frame size: {} bytes", settings_frame.len());
    println!("  Settings: {:?}", settings);
    
    // Parse the settings we just built
    match parser.parse_frame(&settings_frame) {
        Ok((frame, _)) => {
            if let Ok(parsed_settings) = parser.parse_settings(&frame.payload) {
                println!("  Parsed settings: {:?}", parsed_settings);
            }
        }
        Err(e) => eprintln!("✗ Failed to parse settings frame: {:?}", e),
    }
}

fn demo_protocol_detection() {
    println!("=== Protocol Detection Demo ===");
    
    // HTTP/1.1 request
    let http1_data = b"GET / HTTP/1.1\r\n";
    if let Some(detection) = detect_protocol(http1_data) {
        println!("✓ Detected protocol from HTTP/1.1 request: {:?}", detection.protocol);
    }
    
    // HTTP/2 connection preface
    let http2_preface = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";
    if let Some(detection) = detect_protocol(http2_preface) {
        println!("✓ Detected protocol from HTTP/2 preface: {:?}", detection.protocol);
        println!("  Consumed: {} bytes", detection.consumed);
    }
    
    // Invalid data
    let invalid_data = b"INVALID PROTOCOL";
    match detect_protocol(invalid_data) {
        Some(_) => println!("✗ Incorrectly detected protocol"),
        None => println!("✓ Correctly rejected invalid protocol"),
    }
}

fn demo_performance() {
    println!("=== Performance Demonstration ===");
    
    let parser = Http1Parser::new();
    let buffer_pool = BufferPool::new();
    
    // Generate test requests
    let mut requests = Vec::new();
    for i in 0..1000 {
        let request = format!(
            "GET /api/endpoint/{} HTTP/1.1\r\nHost: example.com\r\nX-Request-ID: {}\r\n\r\n",
            i, i
        );
        requests.push(request.into_bytes());
    }
    
    // Warm up
    for request in requests.iter().take(10) {
        let _ = parser.parse_request(request);
    }
    
    // Benchmark parsing
    let start = Instant::now();
    let mut total_bytes = 0;
    let mut successful_parses = 0;
    
    for request in &requests {
        if let Ok((_, consumed)) = parser.parse_request(request) {
            total_bytes += consumed;
            successful_parses += 1;
        }
    }
    
    let duration = start.elapsed();
    let throughput_mbps = (total_bytes as f64 / 1_000_000.0) / duration.as_secs_f64();
    let requests_per_sec = successful_parses as f64 / duration.as_secs_f64();
    
    println!("✓ Parsed {} requests in {:?}", successful_parses, duration);
    println!("  Throughput: {:.2} MB/s", throughput_mbps);
    println!("  Requests/sec: {:.0}", requests_per_sec);
    println!("  Avg time per request: {:.2} µs", 
        duration.as_micros() as f64 / successful_parses as f64
    );
    
    // Demonstrate zero-allocation response building
    println!("\n✓ Zero-allocation response building:");
    let start = Instant::now();
    
    for i in 0..1000 {
        let mut builder = Http1ResponseBuilder::new();
        let _ = builder
            .status(200, "OK")
            .header("X-Request-ID", &i.to_string())
            .body(Some(b"OK"));
    }
    
    let duration = start.elapsed();
    println!("  Built 1000 responses in {:?}", duration);
    println!("  Avg time per response: {:.2} µs", duration.as_micros() as f64 / 1000.0);
}

// Example showing how SIMD optimizations work
fn demonstrate_simd_operations() {
    use angelax_core::utils::simd::*;
    
    println!("\n=== SIMD Operations Demo ===");
    
    // Finding delimiters
    let data = b"Content-Type: application/json\r\nAuthorization: Bearer token";
    let colon_finder = SimdDelimiterFinder::new(b':');
    
    if let Some(pos) = colon_finder.find_in(data) {
        println!("✓ Found colon at position {} using SIMD", pos);
    }
    
    // Finding CRLF sequences
    let crlf_finder = SimdCrlfFinder::new();
    if let Some(pos) = crlf_finder.find_crlf(data) {
        println!("✓ Found CRLF at position {} using SIMD", pos);
    }
    
    // Token validation
    let valid_token = b"Content-Type";
    let invalid_token = b"Content Type"; // Space makes it invalid
    
    println!("✓ Token '{}' is valid: {}", 
        String::from_utf8_lossy(valid_token),
        SimdTokenValidator::is_valid_token(valid_token)
    );
    println!("✓ Token '{}' is valid: {}", 
        String::from_utf8_lossy(invalid_token),
        SimdTokenValidator::is_valid_token(invalid_token)
    );
}
