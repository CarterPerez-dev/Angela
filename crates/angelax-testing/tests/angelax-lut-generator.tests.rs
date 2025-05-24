// This module contains comprehensive tests for the HPACK Huffman LUT generator to ensure correctness and performance.
// It validates tree construction, symbol decoding, edge cases, and the generated lookup table against the RFC 7541 specification.

#[cfg(test)]
mod lut_generator_tests {
    use super::super::lut_generator::*;
    use super::super::tables::*;
    
    /// Test helper to create bit patterns
    fn create_bit_pattern(bits: &[bool]) -> u8 {
        let mut pattern = 0u8;
        for (i, &bit) in bits.iter().enumerate() {
            if bit && i < 8 {
                pattern |= 1 << (7 - i);
            }
        }
        pattern
    }
    
    #[test]
    fn test_huffman_tree_all_symbols() {
        let generator = LutGenerator::new();
        
        // I'm verifying that all symbols from RFC table can be decoded
        for code_entry in RFC7541_STATIC_HUFFMAN_TABLE {
            // Create a bit pattern that starts with this code
            let mut pattern = 0u8;
            if code_entry.bits <= 8 {
                // Shift the code to align with MSB
                pattern = (code_entry.code as u8) << (8 - code_entry.bits);
            }
            
            let entry = generator.decode_pattern(pattern);
            
            // For short codes, we should decode the symbol
            if code_entry.bits <= 8 {
                assert!(entry.num_decoded > 0, 
                    "Failed to decode symbol {} with code {:b}", 
                    code_entry.symbol_id, code_entry.code);
                
                if entry.num_decoded > 0 {
                    let decoded_symbol = entry.symbols[0] as u16;
                    if decoded_symbol <= 255 {
                        assert_eq!(decoded_symbol, code_entry.symbol_id,
                            "Decoded wrong symbol: expected {}, got {}", 
                            code_entry.symbol_id, decoded_symbol);
                    }
                }
            }
        }
    }
    
    #[test]
    fn test_common_ascii_patterns() {
        let generator = LutGenerator::new();
        let lut = generator.generate_lut();
        
        // Test patterns for common ASCII characters
        let test_cases = vec![
            // (pattern, expected_symbol, expected_bits)
            (0b00000000, 48, 5),  // '0' - 5 bits: 00000
            (0b00001000, 49, 5),  // '1' - 5 bits: 00001
            (0b00010000, 50, 5),  // '2' - 5 bits: 00010
            (0b00011000, 97, 5),  // 'a' - 5 bits: 00011
            (0b00100000, 99, 5),  // 'c' - 5 bits: 00100
            (0b00101000, 101, 5), // 'e' - 5 bits: 00101
            (0b00110000, 105, 5), // 'i' - 5 bits: 00110
            (0b00111000, 111, 5), // 'o' - 5 bits: 00111
            (0b01000000, 115, 5), // 's' - 5 bits: 01000
            (0b01001000, 116, 5), // 't' - 5 bits: 01001
            (0b01010000, 32, 6),  // ' ' - 6 bits: 010100
            (0b01010100, 37, 6),  // '%' - 6 bits: 010101
            (0b01011000, 45, 6),  // '-' - 6 bits: 010110
            (0b01011100, 46, 6),  // '.' - 6 bits: 010111
            (0b01100000, 47, 6),  // '/' - 6 bits: 011000
        ];
        
        for (pattern, expected_symbol, expected_bits) in test_cases {
            let entry = &lut[pattern as usize];
            
            assert!(entry.num_decoded > 0, 
                "No symbols decoded for pattern {:08b}", pattern);
            assert_eq!(entry.symbols[0], expected_symbol as u8,
                "Wrong symbol for pattern {:08b}: expected {}, got {}", 
                pattern, expected_symbol, entry.symbols[0]);
            assert_eq!(entry.bits_consumed, expected_bits,
                "Wrong bits consumed for pattern {:08b}: expected {}, got {}", 
                pattern, expected_bits, entry.bits_consumed);
        }
    }
    
    #[test]
    fn test_multiple_symbol_decoding() {
        let generator = LutGenerator::new();
        
        // Pattern with multiple short symbols
        // '0' (00000) + '1' (00001) = 0000000001...
        let pattern = 0b00000000;
        let entry = generator.decode_pattern(pattern);
        
        // Should decode at least one symbol
        assert!(entry.num_decoded >= 1);
        assert_eq!(entry.symbols[0], 48); // '0'
        
        // Pattern: 't' (01001) + 'h' (011000) would be 01001011000...
        // But 'h' is 6 bits, so total is 11 bits, exceeding our 8-bit lookup
        let pattern2 = 0b01001011;
        let entry2 = generator.decode_pattern(pattern2);
        assert!(entry2.num_decoded >= 1);
        assert_eq!(entry2.symbols[0], 116); // 't'
    }
    
    #[test]
    fn test_partial_symbol_handling() {
        let generator = LutGenerator::new();
        
        // Pattern that ends in the middle of a symbol
        // Start of 'A' (0100001) but only 6 bits
        let pattern = 0b01000010;
        let entry = generator.decode_pattern(pattern);
        
        // Should have proper state for continuation
        assert_ne!(entry.next_decoder_state_id, STATE_ERROR);
        
        // If we consumed all 8 bits without completing a symbol
        if entry.num_decoded == 0 {
            assert_eq!(entry.bits_consumed, 8);
            assert!(entry.next_decoder_state_id > 0 && 
                   entry.next_decoder_state_id != STATE_EOS_DECODED);
        }
    }
    
    #[test]
    fn test_state_transitions() {
        let generator = LutGenerator::new();
        
        // Test that FSM states are properly assigned
        let mut state_counts = std::collections::HashMap::new();
        
        for i in 0..256 {
            let entry = generator.decode_pattern(i as u8);
            *state_counts.entry(entry.next_decoder_state_id).or_insert(0) += 1;
        }
        
        // We should have multiple different states
        assert!(state_counts.len() > 1, "Should have multiple FSM states");
        
        // STATE_CONTINUE_LUT should be common (after complete symbols)
        assert!(state_counts.get(&STATE_CONTINUE_LUT).is_some(), 
            "Should have entries returning to LUT");
    }
    
    #[test]
    fn test_error_detection() {
        let generator = LutGenerator::new();
        let lut = generator.generate_lut();
        
        // Verify no entry has invalid state
        for (i, entry) in lut.iter().enumerate() {
            assert_ne!(entry.next_decoder_state_id, STATE_ERROR,
                "Entry {} should not have ERROR state in valid Huffman tree", i);
        }
    }
    
    #[test]
    fn test_bits_consumed_consistency() {
        let generator = LutGenerator::new();
        let lut = generator.generate_lut();
        
        for (i, entry) in lut.iter().enumerate() {
            // Bits consumed should be reasonable
            assert!(entry.bits_consumed <= K_LOOKUP_BITS as u8,
                "Entry {} consumed {} bits, exceeding lookup size {}", 
                i, entry.bits_consumed, K_LOOKUP_BITS);
            
            // If we decoded symbols, we must have consumed bits
            if entry.num_decoded > 0 {
                assert!(entry.bits_consumed > 0,
                    "Entry {} decoded symbols but consumed no bits", i);
            }
        }
    }
    
    #[test]
    fn test_special_symbols() {
        let generator = LutGenerator::new();
        
        // Test patterns that might contain special symbols
        // Note: EOS (256) is 30 bits, so it won't fit in 8-bit lookup
        // But we should handle partial EOS patterns correctly
        
        // Pattern starting with all 1s (potential start of EOS or high symbols)
        let pattern = 0b11111111;
        let entry = generator.decode_pattern(pattern);
        
        // Should not error out
        assert_ne!(entry.next_decoder_state_id, STATE_ERROR);
    }
    
    #[test]
    fn test_code_generation_validity() {
        let generator = LutGenerator::new();
        let lut = generator.generate_lut();
        let code = generator.format_lut_as_rust_code(&lut);
        
        // I'm verifying the generated code has all required elements
        assert!(code.contains("pub const DECODING_LUT"));
        assert!(code.contains("[LutEntry; 256]"));
        
        // Check for proper constants usage
        assert!(code.contains("STATE_CONTINUE_LUT"));
        
        // Verify all entries are present
        let entry_count = code.matches("LutEntry {").count();
        assert_eq!(entry_count, 256, "Should generate exactly 256 entries");
        
        // Check for pattern comments
        assert!(code.contains("// Pattern: 0b00000000"));
        assert!(code.contains("// Pattern: 0b11111111"));
    }
    
    #[test]
    fn test_fsm_generation() {
        let generator = LutGenerator::new();
        
        // Verify FSM states are generated for partial paths
        assert!(!generator.fsm_states.is_empty(), 
            "Should generate FSM states for partial decoding");
        
        // Each state should have exactly 2 transitions (0 and 1)
        for state in &generator.fsm_states {
            assert_eq!(state.transitions.len(), 2,
                "FSM state {} should have exactly 2 transitions", state.id);
            assert!(state.transitions.contains_key(&0));
            assert!(state.transitions.contains_key(&1));
        }
    }
    
    #[test]
    fn test_performance_characteristics() {
        let generator = LutGenerator::new();
        let lut = generator.generate_lut();
        
        // I'm analyzing the performance characteristics of the generated LUT
        let mut single_symbol_count = 0;
        let mut multi_symbol_count = 0;
        let mut partial_count = 0;
        
        for entry in &lut {
            match entry.num_decoded {
                0 => partia
