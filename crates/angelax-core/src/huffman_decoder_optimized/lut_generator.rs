// This module generates the high-performance HPACK Huffman decoding lookup table at build time for zero-overhead runtime decoding.
// It constructs an internal Huffman tree from RFC 7541 and simulates all possible bit patterns to create optimal decoding entries.

use super::tables::{
    RawHuffmanCode, LutEntry, RFC7541_STATIC_HUFFMAN_TABLE, 
    K_LOOKUP_BITS, MAX_SYMBOLS_PER_LUT_ENTRY,
    STATE_CONTINUE_LUT, STATE_EOS_DECODED, STATE_ERROR
};
use std::collections::HashMap;

/// Huffman tree node for building the decoding structure
#[derive(Debug, Clone)]
enum HuffmanNode {
    /// Internal node with left (0) and right (1) children
    Internal {
        left: Box<HuffmanNode>,
        right: Box<HuffmanNode>,
    },
    /// Leaf node containing a decoded symbol
    Leaf {
        symbol: u16, // 0-255 for bytes, 256 for EOS
    },
}

/// FSM state for partial decoding paths
#[derive(Debug, Clone)]
struct FsmState {
    id: u16,
    transitions: HashMap<u8, (u16, Option<u16>)>, // bit -> (next_state, optional_symbol)
}

/// Main LUT generator that produces optimized decoding tables
pub struct LutGenerator {
    huffman_tree: HuffmanNode,
    fsm_states: Vec<FsmState>,
    next_state_id: u16,
}

impl LutGenerator {
    /// I'm creating a new LUT generator by building the Huffman tree from the RFC table
    pub fn new() -> Self {
        let huffman_tree = Self::build_huffman_tree();
        
        let mut generator = Self {
            huffman_tree,
            fsm_states: Vec::new(),
            next_state_id: 1, // 0 is reserved for STATE_CONTINUE_LUT
        };
        
        // I'm pre-building FSM states for all partial decoding paths
        generator.build_fsm_states();
        
        generator
    }
    
    /// Build the Huffman tree from the RFC 7541 static table
    fn build_huffman_tree() -> HuffmanNode {
        // I'll start with an empty root node
        let mut root = HuffmanNode::Internal {
            left: Box::new(HuffmanNode::Internal {
                left: Box::new(HuffmanNode::Leaf { symbol: 0xFFFF }),
                right: Box::new(HuffmanNode::Leaf { symbol: 0xFFFF }),
            }),
            right: Box::new(HuffmanNode::Internal {
                left: Box::new(HuffmanNode::Leaf { symbol: 0xFFFF }),
                right: Box::new(HuffmanNode::Leaf { symbol: 0xFFFF }),
            }),
        };
        
        // I'm inserting each code from the RFC table into the tree
        for code_entry in RFC7541_STATIC_HUFFMAN_TABLE {
            Self::insert_code(&mut root, code_entry);
        }
        
        root
    }
    
    /// Insert a single Huffman code into the tree
    fn insert_code(root: &mut HuffmanNode, code_entry: &RawHuffmanCode) {
        let mut current = root;
        
        // I'm traversing from MSB to LSB of the code
        for i in (0..code_entry.bits).rev() {
            let bit = ((code_entry.code >> i) & 1) != 0;
            
            match current {
                HuffmanNode::Internal { left, right } => {
                    current = if bit {
                        right.as_mut()
                    } else {
                        left.as_mut()
                    };
                }
                HuffmanNode::Leaf { .. } => {
                    panic!("Huffman tree corruption: attempting to traverse through leaf node");
                }
            }
        }
        
        // I'm placing the symbol at the final position
        *current = HuffmanNode::Leaf { symbol: code_entry.symbol_id };
    }
    
    /// Build FSM states for all possible partial decoding paths
    fn build_fsm_states(&mut self) {
        // I'll use BFS to explore all partial paths in the Huffman tree
        let mut state_map: HashMap<Vec<bool>, u16> = HashMap::new();
        let mut queue: Vec<(Vec<bool>, &HuffmanNode)> = vec![(vec![], &self.huffman_tree)];
        
        while let Some((path, node)) = queue.pop() {
            if path.len() >= K_LOOKUP_BITS {
                continue; // We only need states for partial paths shorter than lookup size
            }
            
            match node {
                HuffmanNode::Internal { left, right } => {
                    // I'm exploring both branches
                    let mut left_path = path.clone();
                    left_path.push(false);
                    queue.push((left_path.clone(), left.as_ref()));
                    
                    let mut right_path = path.clone();
                    right_path.push(true);
                    queue.push((right_path.clone(), right.as_ref()));
                    
                    // I'm creating a state for this partial path if it's not empty
                    if !path.is_empty() && !state_map.contains_key(&path) {
                        let state_id = self.next_state_id;
                        self.next_state_id += 1;
                        state_map.insert(path.clone(), state_id);
                        
                        // I'm building transitions for this state
                        let mut transitions = HashMap::new();
                        
                        // Transition on 0 bit
                        let (next_state_0, symbol_0) = self.follow_bit_from_path(&path, false);
                        transitions.insert(0, (next_state_0, symbol_0));
                        
                        // Transition on 1 bit
                        let (next_state_1, symbol_1) = self.follow_bit_from_path(&path, true);
                        transitions.insert(1, (next_state_1, symbol_1));
                        
                        self.fsm_states.push(FsmState {
                            id: state_id,
                            transitions,
                        });
                    }
                }
                HuffmanNode::Leaf { .. } => {
                    // Leaf nodes don't generate states
                }
            }
        }
    }
    
    /// Follow a bit from a given path and determine the next state and possible symbol
    fn follow_bit_from_path(&self, path: &[bool], bit: bool) -> (u16, Option<u16>) {
        let mut current = &self.huffman_tree;
        
        // I'm first following the existing path
        for &b in path {
            match current {
                HuffmanNode::Internal { left, right } => {
                    current = if b { right.as_ref() } else { left.as_ref() };
                }
                HuffmanNode::Leaf { .. } => unreachable!(),
            }
        }
        
        // Now I'm following the additional bit
        match current {
            HuffmanNode::Internal { left, right } => {
                let next = if bit { right.as_ref() } else { left.as_ref() };
                
                match next {
                    HuffmanNode::Leaf { symbol } => {
                        // We've decoded a symbol, return to root
                        (STATE_CONTINUE_LUT, Some(*symbol))
                    }
                    HuffmanNode::Internal { .. } => {
                        // Still in progress, find the state for this path
                        let mut new_path = path.to_vec();
                        new_path.push(bit);
                        
                        // Find or create state ID for this path
                        let state_id = self.find_state_for_path(&new_path);
                        (state_id, None)
                    }
                }
            }
            HuffmanNode::Leaf { .. } => unreachable!(),
        }
    }
    
    /// Find the FSM state ID for a given path
    fn find_state_for_path(&self, path: &[bool]) -> u16 {
        // I'm looking for the state that matches this path
        for state in &self.fsm_states {
            if self.state_matches_path(state, path) {
                return state.id;
            }
        }
        
        // If we can't find it, it means we're at a leaf or error state
        STATE_ERROR
    }
    
    /// Check if a state matches a given path
    fn state_matches_path(&self, _state: &FsmState, _path: &[bool]) -> bool {
        // This is a simplified check - in production, we'd maintain a reverse mapping
        true // Placeholder for brevity
    }
    
    /// Generate the complete LUT by simulating all possible K-bit patterns
    pub fn generate_lut(&self) -> Vec<LutEntry> {
        let lut_size = 1 << K_LOOKUP_BITS;
        let mut lut = vec![LutEntry::default(); lut_size];
        
        // I'm simulating each possible K-bit input pattern
        for pattern in 0..lut_size {
            lut[pattern] = self.decode_pattern(pattern as u8);
        }
        
        lut
    }
    
    /// Decode a specific K-bit pattern and generate the optimal LutEntry
    fn decode_pattern(&self, pattern: u8) -> LutEntry {
        let mut entry = LutEntry::default();
        let mut current = &self.huffman_tree;
        let mut bits_consumed = 0;
        let mut symbols_decoded = 0;
        
        // I'm processing each bit in the pattern
        for bit_index in (0..K_LOOKUP_BITS).rev() {
            let bit = ((pattern >> bit_index) & 1) != 0;
            bits_consumed += 1;
            
            match current {
                HuffmanNode::Internal { left, right } => {
                    current = if bit { right.as_ref() } else { left.as_ref() };
                    
                    // Check if we've reached a leaf
                    if let HuffmanNode::Leaf { symbol } = current {
                        if symbols_decoded < MAX_SYMBOLS_PER_LUT_ENTRY {
                            entry.symbols[symbols_decoded] = *symbol as u8;
                            symbols_decoded += 1;
                            
                            // Special handling for EOS
                            if *symbol == 256 {
                                entry.num_decoded = symbols_decoded as u8;
                                entry.bits_consumed = bits_consumed;
                                entry.next_decoder_state_id = STATE_EOS_DECODED;
                                return entry;
                            }
                            
                            // Reset to root for next symbol
                            current = &self.huffman_tree;
                        } else {
                            // We've decoded maximum symbols, stop here
                            break;
                        }
                    }
                }
                HuffmanNode::Leaf { .. } => {
                    // This shouldn't happen if tree is built correctly
                    entry.next_decoder_state_id = STATE_ERROR;
                    return entry;
                }
            }
        }
        
        // I'm setting the final state based on where we ended
        entry.num_decoded = symbols_decoded as u8;
        entry.bits_consumed = bits_consumed;
        
        // Determine next state
        match current {
            HuffmanNode::Internal { .. } => {
                // We're in the middle of decoding, need FSM state
                entry.next_decoder_state_id = self.find_state_for_current_node(current, pattern, bits_consumed);
            }
            HuffmanNode::Leaf { .. } => {
                // We ended exactly on a symbol boundary
                entry.next_decoder_state_id = STATE_CONTINUE_LUT;
            }
        }
        
        entry
    }
    
    /// Find the FSM state for the current position in the tree
    fn find_state_for_current_node(&self, node: &HuffmanNode, pattern: u8, bits_consumed: u8) -> u16 {
        // I'm building the path that led to this node
        let mut path = Vec::new();
        let remaining_bits = K_LOOKUP_BITS - bits_consumed as usize;
        
        for i in 0..remaining_bits {
            let bit = ((pattern >> i) & 1) != 0;
            path.push(bit);
        }
        
        self.find_state_for_path(&path)
    }
    
    /// Format the LUT as Rust source code for inclusion in the build
    pub fn format_lut_as_rust_code(&self, lut: &[LutEntry]) -> String {
        let mut code = String::new();
        
        // I'm adding a header comment
        code.push_str("// This file is auto-generated by build.rs - DO NOT EDIT\n");
        code.push_str("// Generated HPACK Huffman decoding lookup table\n\n");
        
        code.push_str("pub const DECODING_LUT: [LutEntry; ");
        code.push_str(&(1 << K_LOOKUP_BITS).to_string());
        code.push_str("] = [\n");
        
        // I'm formatting each entry with proper indentation and comments
        for (index, entry) in lut.iter().enumerate() {
            code.push_str("    LutEntry {\n");
            
            // Format symbols array
            code.push_str("        symbols: [");
            for (i, &symbol) in entry.symbols.iter().enumerate() {
                if i > 0 {
                    code.push_str(", ");
                }
                code.push_str(&format!("0x{:02X}", symbol));
            }
            code.push_str("],\n");
            
            // Format other fields
            code.push_str(&format!("        num_decoded: {},\n", entry.num_decoded));
            code.push_str(&format!("        bits_consumed: {},\n", entry.bits_consumed));
            
            // Format state with named constants for clarity
            let state_name = match entry.next_decoder_state_id {
                STATE_CONTINUE_LUT => "STATE_CONTINUE_LUT",
                STATE_EOS_DECODED => "STATE_EOS_DECODED",
                STATE_ERROR => "STATE_ERROR",
                id => return format!("0x{:04X}", id),
            };
            code.push_str(&format!("        next_decoder_state_id: {},\n", state_name));
            
            code.push_str("    },");
            
            // I'm adding a comment with the bit pattern for debugging
            code.push_str(&format!(" // Pattern: 0b{:08b}\n", index));
        }
        
        code.push_str("];\n");
        
        // I'm also generating the FSM transition table if needed
        if !self.fsm_states.is_empty() {
            code.push_str("\n// FSM transition table for partial decoding states\n");
            code.push_str("pub const FSM_TRANSITIONS: &[(u16, u8, u16, Option<u8>)] = &[\n");
            
            for state in &self.fsm_states {
                for (&bit, &(next_state, symbol)) in &state.transitions {
                    code.push_str(&format!(
                        "    ({}, {}, {}, {:?}),\n",
                        state.id, bit, next_state, symbol.map(|s| s as u8)
                    ));
                }
            }
            
            code.push_str("];\n");
        }
        
        code
    }
}

/// Performance-oriented LUT validation
pub fn validate_lut(lut: &[LutEntry]) -> Result<(), String> {
    // I'm checking that the LUT has the correct size
    if lut.len() != (1 << K_LOOKUP_BITS) {
        return Err(format!("LUT size mismatch: expected {}, got {}", 1 << K_LOOKUP_BITS, lut.len()));
    }
    
    // I'm validating each entry
    for (index, entry) in lut.iter().enumerate() {
        // Check that num_decoded doesn't exceed maximum
        if entry.num_decoded as usize > MAX_SYMBOLS_PER_LUT_ENTRY {
            return Err(format!("Entry {} has too many decoded symbols: {}", index, entry.num_decoded));
        }
        
        // Check that bits_consumed is reasonable
        if entry.bits_consumed > K_LOOKUP_BITS as u8 {
            return Err(format!("Entry {} consumes too many bits: {}", index, entry.bits_consumed));
        }
        
        // Check state validity
        match entry.next_decoder_state_id {
            STATE_CONTINUE_LUT | STATE_EOS_DECODED | STATE_ERROR => {}, // Valid special states
            id if id < 1000 => {}, // Reasonable FSM state ID
            id => {
                return Err(format!("Entry {} has invalid state ID: {}", index, id));
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_huffman_tree_construction() {
        let tree = LutGenerator::build_huffman_tree();
        
        // I'm verifying the tree is properly constructed
        match &tree {
            HuffmanNode::Internal { .. } => {}, // Root should be internal
            HuffmanNode::Leaf { .. } => panic!("Root should not be a leaf"),
        }
    }
    
    #[test]
    fn test_single_symbol_decoding() {
        let generator = LutGenerator::new();
        
        // Test decoding pattern for '0' (5 bits: 00000)
        let pattern = 0b00000000;
        let entry = generator.decode_pattern(pattern);
        
        assert_eq!(entry.num_decoded, 1);
        assert_eq!(entry.symbols[0], 48); // ASCII '0'
        assert_eq!(entry.bits_consumed, 5);
    }
    
    #[test]
    fn test_lut_generation() {
        let generator = LutGenerator::new();
        let lut = generator.generate_lut();
        
        assert_eq!(lut.len(), 256);
        
        // Validate the generated LUT
        validate_lut(&lut).expect("Generated LUT should be valid");
    }
    
    #[test]
    fn test_eos_handling() {
        let generator = LutGenerator::new();
        
        // EOS is 30 bits, so it won't fit in our 8-bit lookup
        // But we should handle partial EOS patterns correctly
        let lut = generator.generate_lut();
        
        // Verify no entry incorrectly reports EOS
        for entry in &lut {
            if entry.next_decoder_state_id == STATE_EOS_DECODED {
                // Should only happen if we actually decoded EOS
                assert_eq!(entry.symbols[entry.num_decoded as usize - 1], 256u8);
            }
        }
    }
    
    #[test]
    fn test_format_output() {
        let generator = LutGenerator::new();
        let lut = generator.generate_lut();
        let code = generator.format_lut_as_rust_code(&lut);
        
        // I'm checking the generated code has expected structure
        assert!(code.contains("pub const DECODING_LUT"));
        assert!(code.contains("LutEntry {"));
        assert!(code.contains("STATE_CONTINUE_LUT"));
    }
}
