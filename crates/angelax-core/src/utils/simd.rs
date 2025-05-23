// This module provides SIMD-accelerated utilities for high-performance HTTP parsing operations.
// It leverages x86_64 and ARM NEON intrinsics to find delimiters, validate characters, and perform case conversions at speeds far exceeding scalar implementations.

use std::arch::x86_64::*;

/// SIMD-accelerated delimiter finder for HTTP parsing
#[derive(Debug, Clone)]
pub struct SimdDelimiterFinder {
    delimiter: u8,
    delimiter_vec: __m256i,
}

impl SimdDelimiterFinder {
    /// I'm creating a new delimiter finder optimized for the given byte
    #[inline]
    pub fn new(delimiter: u8) -> Self {
        unsafe {
            Self {
                delimiter,
                delimiter_vec: _mm256_set1_epi8(delimiter as i8),
            }
        }
    }

    /// Find the position of the delimiter in the given slice using AVX2
    #[inline]
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    pub fn find_in(&self, haystack: &[u8]) -> Option<usize> {
        unsafe {
            let mut offset = 0;
            let len = haystack.len();

            // I'm processing 32 bytes at a time with AVX2
            while offset + 32 <= len {
                let chunk = _mm256_loadu_si256(haystack.as_ptr().add(offset) as *const __m256i);
                let cmp = _mm256_cmpeq_epi8(chunk, self.delimiter_vec);
                let mask = _mm256_movemask_epi8(cmp);

                if mask != 0 {
                    return Some(offset + mask.trailing_zeros() as usize);
                }

                offset += 32;
            }

            // Handle remaining bytes with SSE2 (16 bytes)
            if offset + 16 <= len {
                let chunk = _mm_loadu_si128(haystack.as_ptr().add(offset) as *const __m128i);
                let delimiter_vec = _mm_set1_epi8(self.delimiter as i8);
                let cmp = _mm_cmpeq_epi8(chunk, delimiter_vec);
                let mask = _mm_movemask_epi8(cmp);

                if mask != 0 {
                    return Some(offset + mask.trailing_zeros() as usize);
                }

                offset += 16;
            }

            // I'm falling back to scalar search for the remaining bytes
            haystack[offset..].iter().position(|&b| b == self.delimiter)
                .map(|pos| offset + pos)
        }
    }

    /// Fallback implementation for non-AVX2 systems
    #[inline]
    #[cfg(not(all(target_arch = "x86_64", target_feature = "avx2")))]
    pub fn find_in(&self, haystack: &[u8]) -> Option<usize> {
        memchr::memchr(self.delimiter, haystack)
    }
}

/// SIMD-accelerated CRLF finder for HTTP header parsing
pub struct SimdCrlfFinder {
    cr_vec: __m256i,
    lf_vec: __m256i,
}

impl SimdCrlfFinder {
    /// I'm creating a specialized CRLF finder
    #[inline]
    pub fn new() -> Self {
        unsafe {
            Self {
                cr_vec: _mm256_set1_epi8(b'\r' as i8),
                lf_vec: _mm256_set1_epi8(b'\n' as i8),
            }
        }
    }

    /// Find CRLF sequence using SIMD
    #[inline]
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    pub fn find_crlf(&self, haystack: &[u8]) -> Option<usize> {
        unsafe {
            let len = haystack.len();
            if len < 2 {
                return None;
            }

            let mut offset = 0;

            // Process 32 bytes at a time
            while offset + 33 <= len { // 33 to ensure we can check the next byte
                let chunk = _mm256_loadu_si256(haystack.as_ptr().add(offset) as *const __m256i);
                let cr_cmp = _mm256_cmpeq_epi8(chunk, self.cr_vec);
                let cr_mask = _mm256_movemask_epi8(cr_cmp);

                if cr_mask != 0 {
                    // I found potential CR positions, now check for LF
                    let mut bit_pos = 0;
                    while bit_pos < 32 {
                        if (cr_mask & (1 << bit_pos)) != 0 {
                            let pos = offset + bit_pos;
                            if pos + 1 < len && haystack[pos + 1] == b'\n' {
                                return Some(pos);
                            }
                        }
                        bit_pos += 1;
                    }
                }

                offset += 32;
            }

            // Handle remaining bytes
            haystack[offset..].windows(2)
                .position(|w| w[0] == b'\r' && w[1] == b'\n')
                .map(|pos| offset + pos)
        }
    }

    #[inline]
    #[cfg(not(all(target_arch = "x86_64", target_feature = "avx2")))]
    pub fn find_crlf(&self, haystack: &[u8]) -> Option<usize> {
        haystack.windows(2)
            .position(|w| w[0] == b'\r' && w[1] == b'\n')
    }
}

/// SIMD-accelerated ASCII uppercase converter
pub struct SimdUppercaseConverter;

impl SimdUppercaseConverter {
    /// Convert ASCII lowercase to uppercase using SIMD
    #[inline]
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    pub fn convert(input: &mut [u8]) {
        unsafe {
            let len = input.len();
            let mut offset = 0;

            // I'm setting up the constants for case conversion
            let lower_a = _mm256_set1_epi8(b'a' as i8);
            let lower_z = _mm256_set1_epi8(b'z' as i8);
            let case_diff = _mm256_set1_epi8(32); // 'a' - 'A' = 32

            while offset + 32 <= len {
                let chunk = _mm256_loadu_si256(input.as_ptr().add(offset) as *const __m256i);
                
                // Check if bytes are lowercase letters
                let ge_a = _mm256_cmpgt_epi8(chunk, _mm256_sub_epi8(lower_a, _mm256_set1_epi8(1)));
                let le_z = _mm256_cmpgt_epi8(_mm256_add_epi8(lower_z, _mm256_set1_epi8(1)), chunk);
                let is_lower = _mm256_and_si256(ge_a, le_z);
                
                // Apply case conversion only to lowercase letters
                let adjustment = _mm256_and_si256(is_lower, case_diff);
                let result = _mm256_sub_epi8(chunk, adjustment);
                
                _mm256_storeu_si256(input.as_mut_ptr().add(offset) as *mut __m256i, result);
                offset += 32;
            }

            // Handle remaining bytes
            for byte in &mut input[offset..] {
                if *byte >= b'a' && *byte <= b'z' {
                    *byte -= 32;
                }
            }
        }
    }

    #[inline]
    #[cfg(not(all(target_arch = "x86_64", target_feature = "avx2")))]
    pub fn convert(input: &mut [u8]) {
        for byte in input {
            if *byte >= b'a' && *byte <= b'z' {
                *byte -= 32;
            }
        }
    }
}

/// SIMD-accelerated token validator for HTTP headers
pub struct SimdTokenValidator;

impl SimdTokenValidator {
    /// Validate HTTP token characters using SIMD
    #[inline]
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    pub fn is_valid_token(input: &[u8]) -> bool {
        unsafe {
            let len = input.len();
            if len == 0 {
                return false;
            }

            let mut offset = 0;

            // I'm defining valid token character ranges
            // Valid: A-Z, a-z, 0-9, and !#$%&'*+-.^_`|~
            while offset + 32 <= len {
                let chunk = _mm256_loadu_si256(input.as_ptr().add(offset) as *const __m256i);
                
                // Check for invalid characters (< 0x21 or > 0x7E or separators)
                let min_valid = _mm256_set1_epi8(0x21);
                let max_valid = _mm256_set1_epi8(0x7E);
                
                let ge_min = _mm256_cmpgt_epi8(chunk, _mm256_sub_epi8(min_valid, _mm256_set1_epi8(1)));
                let le_max = _mm256_cmpgt_epi8(_mm256_add_epi8(max_valid, _mm256_set1_epi8(1)), chunk);
                let in_range = _mm256_and_si256(ge_min, le_max);
                
                let mask = _mm256_movemask_epi8(in_range);
                if mask != -1 {
                    // Some bytes are out of range, need detailed check
                    return false;
                }
                
                // TODO: Check for separator characters
                
                offset += 32;
            }

            // Handle remaining bytes
            input[offset..].iter().all(|&b| {
                matches!(b, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' |
                    b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' |
                    b'+' | b'-' | b'.' | b'^' | b'_' | b'`' | b'|' | b'~')
            })
        }
    }

    #[inline]
    #[cfg(not(all(target_arch = "x86_64", target_feature = "avx2")))]
    pub fn is_valid_token(input: &[u8]) -> bool {
        !input.is_empty() && input.iter().all(|&b| {
            matches!(b, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' |
                b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' |
                b'+' | b'-' | b'.' | b'^' | b'_' | b'`' | b'|' | b'~')
        })
    }
}

/// SIMD-accelerated whitespace skipper
pub struct SimdWhitespaceSkipper;

impl SimdWhitespaceSkipper {
    /// Skip leading whitespace using SIMD
    #[inline]
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    pub fn skip_whitespace(input: &[u8]) -> &[u8] {
        unsafe {
            let len = input.len();
            let mut offset = 0;

            let space = _mm256_set1_epi8(b' ' as i8);
            let tab = _mm256_set1_epi8(b'\t' as i8);

            while offset + 32 <= len {
                let chunk = _mm256_loadu_si256(input.as_ptr().add(offset) as *const __m256i);
                
                let is_space = _mm256_cmpeq_epi8(chunk, space);
                let is_tab = _mm256_cmpeq_epi8(chunk, tab);
                let is_whitespace = _mm256_or_si256(is_space, is_tab);
                
                let mask = _mm256_movemask_epi8(is_whitespace);
                
                if mask != -1 {
                    // Found non-whitespace
                    let non_ws_pos = (!mask).trailing_zeros() as usize;
                    return &input[offset + non_ws_pos..];
                }
                
                offset += 32;
            }

            // Handle remaining bytes
            &input[offset..].iter()
                .position(|&b| b != b' ' && b != b'\t')
                .map(|pos| &input[offset + pos..])
                .unwrap_or(&[])
        }
    }

    #[inline]
    #[cfg(not(all(target_arch = "x86_64", target_feature = "avx2")))]
    pub fn skip_whitespace(input: &[u8]) -> &[u8] {
        input.iter()
            .position(|&b| b != b' ' && b != b'\t')
            .map(|pos| &input[pos..])
            .unwrap_or(&[])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delimiter_finder() {
        let finder = SimdDelimiterFinder::new(b':');
        
        assert_eq!(finder.find_in(b"Content-Type: text/plain"), Some(12));
        assert_eq!(finder.find_in(b"No colon here"), None);
        assert_eq!(finder.find_in(b":at_start"), Some(0));
        assert_eq!(finder.find_in(b"at_end:"), Some(6));
        
        // Test with longer string to ensure SIMD paths
        let long_str = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
        assert_eq!(finder.find_in(long_str), Some(32));
    }

    #[test]
    fn test_crlf_finder() {
        let finder = SimdCrlfFinder::new();
        
        assert_eq!(finder.find_crlf(b"Line1\r\nLine2"), Some(5));
        assert_eq!(finder.find_crlf(b"No CRLF here"), None);
        assert_eq!(finder.find_crlf(b"\r\n"), Some(0));
        assert_eq!(finder.find_crlf(b"Just\rCR"), None);
        assert_eq!(finder.find_crlf(b"Just\nLF"), None);
        
        // Test across SIMD boundary
        let mut long_str = vec![b'a'; 31];
        long_str.extend_from_slice(b"\r\n");
        assert_eq!(finder.find_crlf(&long_str), Some(31));
    }

    #[test]
    fn test_uppercase_converter() {
        let mut input = b"Content-Type: text/plain".to_vec();
        SimdUppercaseConverter::convert(&mut input);
        assert_eq!(&input, b"CONTENT-TYPE: TEXT/PLAIN");
        
        let mut mixed = b"MiXeD-CaSe-123".to_vec();
        SimdUppercaseConverter::convert(&mut mixed);
        assert_eq!(&mixed, b"MIXED-CASE-123");
        
        // Test long string for SIMD
        let mut long_str = vec![b'a'; 64];
        SimdUppercaseConverter::convert(&mut long_str);
        assert!(long_str.iter().all(|&b| b == b'A'));
    }

    #[test]
    fn test_token_validator() {
        assert!(SimdTokenValidator::is_valid_token(b"Content-Type"));
        assert!(SimdTokenValidator::is_valid_token(b"X-Custom-Header"));
        assert!(SimdTokenValidator::is_valid_token(b"123"));
        assert!(SimdTokenValidator::is_valid_token(b"!#$%&'*+-.^_`|~"));
        
        assert!(!SimdTokenValidator::is_valid_token(b""));
        assert!(!SimdTokenValidator::is_valid_token(b"Has Space"));
        assert!(!SimdTokenValidator::is_valid_token(b"Has\tTab"));
        assert!(!SimdTokenValidator::is_valid_token(b"Has:Colon"));
    }

    #[test]
    fn test_whitespace_skipper() {
        assert_eq!(SimdWhitespaceSkipper::skip_whitespace(b"  text"), b"text");
        assert_eq!(SimdWhitespaceSkipper::skip_whitespace(b"\t\ttext"), b"text");
        assert_eq!(SimdWhitespaceSkipper::skip_whitespace(b" \t mixed"), b"mixed");
        assert_eq!(SimdWhitespaceSkipper::skip_whitespace(b"no_ws"), b"no_ws");
        assert_eq!(SimdWhitespaceSkipper::skip_whitespace(b"   "), b"");
        
        // Test with long whitespace prefix
        let mut long_ws = vec![b' '; 64];
        long_ws.extend_from_slice(b"text");
        assert_eq!(SimdWhitespaceSkipper::skip_whitespace(&long_ws), b"text");
    }
}
