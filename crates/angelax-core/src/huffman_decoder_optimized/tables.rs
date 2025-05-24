// In crates/angelax-core/src/huffman_decoder_optimized/tables.rs

#[derive(Debug, Clone, Copy)]
pub struct RawHuffmanCode {
    pub symbol_id: u16, // 0-255 for bytes, 256 for EOS
    pub code: u32,      // The Huffman code value (from "code as hex" in RFC)
    pub bits: u8,       // Number of bits in the code
}

// This is the HPACK Static Huffman Table from RFC 7541, Appendix B.
pub const RFC7541_STATIC_HUFFMAN_TABLE: &[RawHuffmanCode] = &[
    RawHuffmanCode { symbol_id: 0, code: 0x1ff8, bits: 13 },
    RawHuffmanCode { symbol_id: 1, code: 0x7fffd8, bits: 23 },
    RawHuffmanCode { symbol_id: 2, code: 0xfffffe2, bits: 28 },
    RawHuffmanCode { symbol_id: 3, code: 0xfffffe3, bits: 28 },
    RawHuffmanCode { symbol_id: 4, code: 0xfffffe4, bits: 28 },
    RawHuffmanCode { symbol_id: 5, code: 0xfffffe5, bits: 28 },
    RawHuffmanCode { symbol_id: 6, code: 0xfffffe6, bits: 28 },
    RawHuffmanCode { symbol_id: 7, code: 0xfffffe7, bits: 28 },
    RawHuffmanCode { symbol_id: 8, code: 0xfffffe8, bits: 28 },
    RawHuffmanCode { symbol_id: 9, code: 0xffffea, bits: 24 },
    RawHuffmanCode { symbol_id: 10, code: 0x3ffffffc, bits: 30 }, // LF ('\n')
    RawHuffmanCode { symbol_id: 11, code: 0xfffffe9, bits: 28 },
    RawHuffmanCode { symbol_id: 12, code: 0xfffffea, bits: 28 },
    RawHuffmanCode { symbol_id: 13, code: 0x3ffffffd, bits: 30 }, // CR ('\r')
    RawHuffmanCode { symbol_id: 14, code: 0xfffffeb, bits: 28 },
    RawHuffmanCode { symbol_id: 15, code: 0xfffffec, bits: 28 },
    RawHuffmanCode { symbol_id: 16, code: 0xfffffed, bits: 28 },
    RawHuffmanCode { symbol_id: 17, code: 0xfffffee, bits: 28 },
    RawHuffmanCode { symbol_id: 18, code: 0xfffffef, bits: 28 },
    RawHuffmanCode { symbol_id: 19, code: 0xffffff0, bits: 28 },
    RawHuffmanCode { symbol_id: 20, code: 0xffffff1, bits: 28 },
    RawHuffmanCode { symbol_id: 21, code: 0xffffff2, bits: 28 },
    RawHuffmanCode { symbol_id: 22, code: 0x3ffffffe, bits: 30 },
    RawHuffmanCode { symbol_id: 23, code: 0xffffff3, bits: 28 },
    RawHuffmanCode { symbol_id: 24, code: 0xffffff4, bits: 28 },
    RawHuffmanCode { symbol_id: 25, code: 0xffffff5, bits: 28 },
    RawHuffmanCode { symbol_id: 26, code: 0xffffff6, bits: 28 },
    RawHuffmanCode { symbol_id: 27, code: 0xffffff7, bits: 28 },
    RawHuffmanCode { symbol_id: 28, code: 0xffffff8, bits: 28 },
    RawHuffmanCode { symbol_id: 29, code: 0xffffff9, bits: 28 },
    RawHuffmanCode { symbol_id: 30, code: 0xffffffa, bits: 28 },
    RawHuffmanCode { symbol_id: 31, code: 0xffffffb, bits: 28 },
    RawHuffmanCode { symbol_id: 32, code: 0x14, bits: 6 },      // ' '
    RawHuffmanCode { symbol_id: 33, code: 0x3f8, bits: 10 },    // '!'
    RawHuffmanCode { symbol_id: 34, code: 0x3f9, bits: 10 },    // '"'
    RawHuffmanCode { symbol_id: 35, code: 0xffa, bits: 12 },    // '#'
    RawHuffmanCode { symbol_id: 36, code: 0x1ff9, bits: 13 },   // '$'
    RawHuffmanCode { symbol_id: 37, code: 0x15, bits: 6 },      // '%'
    RawHuffmanCode { symbol_id: 38, code: 0xf8, bits: 8 },      // '&'
    RawHuffmanCode { symbol_id: 39, code: 0x7fa, bits: 11 },    // '''
    RawHuffmanCode { symbol_id: 40, code: 0x3fa, bits: 10 },    // '('
    RawHuffmanCode { symbol_id: 41, code: 0x3fb, bits: 10 },    // ')'
    RawHuffmanCode { symbol_id: 42, code: 0xf9, bits: 8 },      // '*'
    RawHuffmanCode { symbol_id: 43, code: 0x7fb, bits: 11 },    // '+'
    RawHuffmanCode { symbol_id: 44, code: 0xfa, bits: 8 },      // ','
    RawHuffmanCode { symbol_id: 45, code: 0x16, bits: 6 },      // '-'
    RawHuffmanCode { symbol_id: 46, code: 0x17, bits: 6 },      // '.'
    RawHuffmanCode { symbol_id: 47, code: 0x18, bits: 6 },      // '/'
    RawHuffmanCode { symbol_id: 48, code: 0x0, bits: 5 },       // '0'
    RawHuffmanCode { symbol_id: 49, code: 0x1, bits: 5 },       // '1'
    RawHuffmanCode { symbol_id: 50, code: 0x2, bits: 5 },       // '2'
    RawHuffmanCode { symbol_id: 51, code: 0x19, bits: 6 },      // '3'
    RawHuffmanCode { symbol_id: 52, code: 0x1a, bits: 6 },      // '4'
    RawHuffmanCode { symbol_id: 53, code: 0x1b, bits: 6 },      // '5'
    RawHuffmanCode { symbol_id: 54, code: 0x1c, bits: 6 },      // '6'
    RawHuffmanCode { symbol_id: 55, code: 0x1d, bits: 6 },      // '7'
    RawHuffmanCode { symbol_id: 56, code: 0x1e, bits: 6 },      // '8'
    RawHuffmanCode { symbol_id: 57, code: 0x1f, bits: 6 },      // '9'
    RawHuffmanCode { symbol_id: 58, code: 0x5c, bits: 7 },      // ':'
    RawHuffmanCode { symbol_id: 59, code: 0xfb, bits: 8 },      // ';'
    RawHuffmanCode { symbol_id: 60, code: 0x7ffc, bits: 15 },   // '<'
    RawHuffmanCode { symbol_id: 61, code: 0x20, bits: 6 },      // '='
    RawHuffmanCode { symbol_id: 62, code: 0xffb, bits: 12 },    // '>'
    RawHuffmanCode { symbol_id: 63, code: 0x3fc, bits: 10 },    // '?'
    RawHuffmanCode { symbol_id: 64, code: 0x1ffa, bits: 13 },   // '@'
    RawHuffmanCode { symbol_id: 65, code: 0x21, bits: 6 },      // 'A'
    RawHuffmanCode { symbol_id: 66, code: 0x5d, bits: 7 },      // 'B'
    RawHuffmanCode { symbol_id: 67, code: 0x5e, bits: 7 },      // 'C'
    RawHuffmanCode { symbol_id: 68, code: 0x5f, bits: 7 },      // 'D'
    RawHuffmanCode { symbol_id: 69, code: 0x60, bits: 7 },      // 'E'
    RawHuffmanCode { symbol_id: 70, code: 0x61, bits: 7 },      // 'F'
    RawHuffmanCode { symbol_id: 71, code: 0x62, bits: 7 },      // 'G'
    RawHuffmanCode { symbol_id: 72, code: 0x63, bits: 7 },      // 'H'
    RawHuffmanCode { symbol_id: 73, code: 0x64, bits: 7 },      // 'I'
    RawHuffmanCode { symbol_id: 74, code: 0x65, bits: 7 },      // 'J'
    RawHuffmanCode { symbol_id: 75, code: 0x66, bits: 7 },      // 'K'
    RawHuffmanCode { symbol_id: 76, code: 0x67, bits: 7 },      // 'L'
    RawHuffmanCode { symbol_id: 77, code: 0x68, bits: 7 },      // 'M'
    RawHuffmanCode { symbol_id: 78, code: 0x69, bits: 7 },      // 'N'
    RawHuffmanCode { symbol_id: 79, code: 0x6a, bits: 7 },      // 'O'
    RawHuffmanCode { symbol_id: 80, code: 0x6b, bits: 7 },      // 'P'
    RawHuffmanCode { symbol_id: 81, code: 0x6c, bits: 7 },      // 'Q'
    RawHuffmanCode { symbol_id: 82, code: 0x6d, bits: 7 },      // 'R'
    RawHuffmanCode { symbol_id: 83, code: 0x6e, bits: 7 },      // 'S'
    RawHuffmanCode { symbol_id: 84, code: 0x6f, bits: 7 },      // 'T'
    RawHuffmanCode { symbol_id: 85, code: 0x70, bits: 7 },      // 'U'
    RawHuffmanCode { symbol_id: 86, code: 0x71, bits: 7 },      // 'V'
    RawHuffmanCode { symbol_id: 87, code: 0x72, bits: 7 },      // 'W'
    RawHuffmanCode { symbol_id: 88, code: 0xfc, bits: 8 },      // 'X'
    RawHuffmanCode { symbol_id: 89, code: 0x73, bits: 7 },      // 'Y'
    RawHuffmanCode { symbol_id: 90, code: 0xfd, bits: 8 },      // 'Z'
    RawHuffmanCode { symbol_id: 91, code: 0x1ffb, bits: 13 },   // '['
    RawHuffmanCode { symbol_id: 92, code: 0x7fff0, bits: 19 },  // '\'
    RawHuffmanCode { symbol_id: 93, code: 0x1ffc, bits: 13 },   // ']'
    RawHuffmanCode { symbol_id: 94, code: 0x3ffc, bits: 14 },   // '^'
    RawHuffmanCode { symbol_id: 95, code: 0x22, bits: 6 },      // '_'
    RawHuffmanCode { symbol_id: 96, code: 0x7ffd, bits: 15 },   // '`'
    RawHuffmanCode { symbol_id: 97, code: 0x3, bits: 5 },       // 'a'
    RawHuffmanCode { symbol_id: 98, code: 0x23, bits: 6 },      // 'b'
    RawHuffmanCode { symbol_id: 99, code: 0x4, bits: 5 },       // 'c'
    RawHuffmanCode { symbol_id: 100, code: 0x24, bits: 6 },     // 'd'
    RawHuffmanCode { symbol_id: 101, code: 0x5, bits: 5 },      // 'e'
    RawHuffmanCode { symbol_id: 102, code: 0x25, bits: 6 },     // 'f'
    RawHuffmanCode { symbol_id: 103, code: 0x26, bits: 6 },     // 'g'
    RawHuffmanCode { symbol_id: 104, code: 0x27, bits: 6 },     // 'h'
    RawHuffmanCode { symbol_id: 105, code: 0x6, bits: 5 },      // 'i'
    RawHuffmanCode { symbol_id: 106, code: 0x74, bits: 7 },     // 'j'
    RawHuffmanCode { symbol_id: 107, code: 0x75, bits: 7 },     // 'k'
    RawHuffmanCode { symbol_id: 108, code: 0x28, bits: 6 },     // 'l'
    RawHuffmanCode { symbol_id: 109, code: 0x29, bits: 6 },     // 'm'
    RawHuffmanCode { symbol_id: 110, code: 0x2a, bits: 6 },     // 'n'
    RawHuffmanCode { symbol_id: 111, code: 0x7, bits: 5 },      // 'o'
    RawHuffmanCode { symbol_id: 112, code: 0x2b, bits: 6 },     // 'p'
    RawHuffmanCode { symbol_id: 113, code: 0x76, bits: 7 },     // 'q'
    RawHuffmanCode { symbol_id: 114, code: 0x2c, bits: 6 },     // 'r'
    RawHuffmanCode { symbol_id: 115, code: 0x8, bits: 5 },      // 's'
    RawHuffmanCode { symbol_id: 116, code: 0x9, bits: 5 },      // 't'
    RawHuffmanCode { symbol_id: 117, code: 0x2d, bits: 6 },     // 'u'
    RawHuffmanCode { symbol_id: 118, code: 0x77, bits: 7 },     // 'v'
    RawHuffmanCode { symbol_id: 119, code: 0x78, bits: 7 },     // 'w'
    RawHuffmanCode { symbol_id: 120, code: 0x79, bits: 7 },     // 'x'
    RawHuffmanCode { symbol_id: 121, code: 0x7a, bits: 7 },     // 'y'
    RawHuffmanCode { symbol_id: 122, code: 0x7b, bits: 7 },     // 'z'
    RawHuffmanCode { symbol_id: 123, code: 0x7ffe, bits: 15 },  // '{'
    RawHuffmanCode { symbol_id: 124, code: 0x7fc, bits: 11 },   // '|'
    RawHuffmanCode { symbol_id: 125, code: 0x3ffd, bits: 14 },  // '}'
    RawHuffmanCode { symbol_id: 126, code: 0x1ffd, bits: 13 },  // '~'
    RawHuffmanCode { symbol_id: 127, code: 0xffffffc, bits: 28 },
    RawHuffmanCode { symbol_id: 128, code: 0xfffe6, bits: 20 },
    RawHuffmanCode { symbol_id: 129, code: 0x3fffd2, bits: 22 },
    RawHuffmanCode { symbol_id: 130, code: 0xfffe7, bits: 20 },
    RawHuffmanCode { symbol_id: 131, code: 0xfffe8, bits: 20 },
    RawHuffmanCode { symbol_id: 132, code: 0x3fffd3, bits: 22 },
    RawHuffmanCode { symbol_id: 133, code: 0x3fffd4, bits: 22 },
    RawHuffmanCode { symbol_id: 134, code: 0x3fffd5, bits: 22 },
    RawHuffmanCode { symbol_id: 135, code: 0x7fffd9, bits: 23 },
    RawHuffmanCode { symbol_id: 136, code: 0x3fffd6, bits: 22 },
    RawHuffmanCode { symbol_id: 137, code: 0x7fffda, bits: 23 },
    RawHuffmanCode { symbol_id: 138, code: 0x7fffdb, bits: 23 },
    RawHuffmanCode { symbol_id: 139, code: 0x7fffdc, bits: 23 },
    RawHuffmanCode { symbol_id: 140, code: 0x7fffdd, bits: 23 },
    RawHuffmanCode { symbol_id: 141, code: 0x7fffde, bits: 23 },
    RawHuffmanCode { symbol_id: 142, code: 0xffffeb, bits: 24 },
    RawHuffmanCode { symbol_id: 143, code: 0x7fffdf, bits: 23 },
    RawHuffmanCode { symbol_id: 144, code: 0xffffec, bits: 24 },
    RawHuffmanCode { symbol_id: 145, code: 0xffffed, bits: 24 },
    RawHuffmanCode { symbol_id: 146, code: 0x3fffd7, bits: 22 },
    RawHuffmanCode { symbol_id: 147, code: 0x7fffe0, bits: 23 },
    RawHuffmanCode { symbol_id: 148, code: 0xffffee, bits: 24 },
    RawHuffmanCode { symbol_id: 149, code: 0x7fffe1, bits: 23 },
    RawHuffmanCode { symbol_id: 150, code: 0x7fffe2, bits: 23 },
    RawHuffmanCode { symbol_id: 151, code: 0x7fffe3, bits: 23 },
    RawHuffmanCode { symbol_id: 152, code: 0x7fffe4, bits: 23 },
    RawHuffmanCode { symbol_id: 153, code: 0x1fffdc, bits: 21 },
    RawHuffmanCode { symbol_id: 154, code: 0x3fffd8, bits: 22 },
    RawHuffmanCode { symbol_id: 155, code: 0x7fffe5, bits: 23 },
    RawHuffmanCode { symbol_id: 156, code: 0x3fffd9, bits: 22 },
    RawHuffmanCode { symbol_id: 157, code: 0x7fffe6, bits: 23 },
    RawHuffmanCode { symbol_id: 158, code: 0x7fffe7, bits: 23 },
    RawHuffmanCode { symbol_id: 159, code: 0xffffef, bits: 24 },
    RawHuffmanCode { symbol_id: 160, code: 0x3fffda, bits: 22 },
    RawHuffmanCode { symbol_id: 161, code: 0x1fffdd, bits: 21 },
    RawHuffmanCode { symbol_id: 162, code: 0xfffe9, bits: 20 },
    RawHuffmanCode { symbol_id: 163, code: 0x3fffdb, bits: 22 },
    RawHuffmanCode { symbol_id: 164, code: 0x3fffdc, bits: 22 },
    RawHuffmanCode { symbol_id: 165, code: 0x7fffe8, bits: 23 },
    RawHuffmanCode { symbol_id: 166, code: 0x7fffe9, bits: 23 },
    RawHuffmanCode { symbol_id: 167, code: 0x1fffde, bits: 21 },
    RawHuffmanCode { symbol_id: 168, code: 0x7fffea, bits: 23 },
    RawHuffmanCode { symbol_id: 169, code: 0x3fffdd, bits: 22 },
    RawHuffmanCode { symbol_id: 170, code: 0x3fffde, bits: 22 },
    RawHuffmanCode { symbol_id: 171, code: 0xfffff0, bits: 24 },
    RawHuffmanCode { symbol_id: 172, code: 0x1fffdf, bits: 21 },
    RawHuffmanCode { symbol_id: 173, code: 0x3fffdf, bits: 22 },
    RawHuffmanCode { symbol_id: 174, code: 0x7fffeb, bits: 23 },
    RawHuffmanCode { symbol_id: 175, code: 0x7fffec, bits: 23 },
    RawHuffmanCode { symbol_id: 176, code: 0x1fffe0, bits: 21 },
    RawHuffmanCode { symbol_id: 177, code: 0x1fffe1, bits: 21 },
    RawHuffmanCode { symbol_id: 178, code: 0x3fffe0, bits: 22 },
    RawHuffmanCode { symbol_id: 179, code: 0x1fffe2, bits: 21 },
    RawHuffmanCode { symbol_id: 180, code: 0x7fffed, bits: 23 },
    RawHuffmanCode { symbol_id: 181, code: 0x3fffe1, bits: 22 },
    RawHuffmanCode { symbol_id: 182, code: 0x7fffee, bits: 23 },
    RawHuffmanCode { symbol_id: 183, code: 0x7fffef, bits: 23 },
    RawHuffmanCode { symbol_id: 184, code: 0xfffea, bits: 20 },
    RawHuffmanCode { symbol_id: 185, code: 0x3fffe2, bits: 22 },
    RawHuffmanCode { symbol_id: 186, code: 0x3fffe3, bits: 22 },
    RawHuffmanCode { symbol_id: 187, code: 0x3fffe4, bits: 22 },
    RawHuffmanCode { symbol_id: 188, code: 0x7ffff0, bits: 23 },
    RawHuffmanCode { symbol_id: 189, code: 0x3fffe5, bits: 22 },
    RawHuffmanCode { symbol_id: 190, code: 0x3fffe6, bits: 22 },
    RawHuffmanCode { symbol_id: 191, code: 0x7ffff1, bits: 23 },
    RawHuffmanCode { symbol_id: 192, code: 0x3ffffe0, bits: 26 },
    RawHuffmanCode { symbol_id: 193, code: 0x3ffffe1, bits: 26 },
    RawHuffmanCode { symbol_id: 194, code: 0xfffeb, bits: 20 },
    RawHuffmanCode { symbol_id: 195, code: 0x7fff1, bits: 19 },
    RawHuffmanCode { symbol_id: 196, code: 0x3fffe7, bits: 22 },
    RawHuffmanCode { symbol_id: 197, code: 0x7ffff2, bits: 23 },
    RawHuffmanCode { symbol_id: 198, code: 0x3fffe8, bits: 22 },
    RawHuffmanCode { symbol_id: 199, code: 0x1ffffec, bits: 25 },
    RawHuffmanCode { symbol_id: 200, code: 0x3ffffe2, bits: 26 },
    RawHuffmanCode { symbol_id: 201, code: 0x3ffffe3, bits: 26 },
    RawHuffmanCode { symbol_id: 202, code: 0x3ffffe4, bits: 26 },
    RawHuffmanCode { symbol_id: 203, code: 0x7ffffde, bits: 27 },
    RawHuffmanCode { symbol_id: 204, code: 0x7ffffdf, bits: 27 },
    RawHuffmanCode { symbol_id: 205, code: 0x3ffffe5, bits: 26 },
    RawHuffmanCode { symbol_id: 206, code: 0xfffff1, bits: 24 },
    RawHuffmanCode { symbol_id: 207, code: 0x1ffffed, bits: 25 },
    RawHuffmanCode { symbol_id: 208, code: 0x7fff2, bits: 19 },
    RawHuffmanCode { symbol_id: 209, code: 0x1fffe3, bits: 21 },
    RawHuffmanCode { symbol_id: 210, code: 0x3ffffe6, bits: 26 },
    RawHuffmanCode { symbol_id: 211, code: 0x7ffffe0, bits: 27 },
    RawHuffmanCode { symbol_id: 212, code: 0x7ffffe1, bits: 27 },
    RawHuffmanCode { symbol_id: 213, code: 0x3ffffe7, bits: 26 },
    RawHuffmanCode { symbol_id: 214, code: 0x7ffffe2, bits: 27 },
    RawHuffmanCode { symbol_id: 215, code: 0xfffff2, bits: 24 },
    RawHuffmanCode { symbol_id: 216, code: 0x1fffe4, bits: 21 },
    RawHuffmanCode { symbol_id: 217, code: 0x1fffe5, bits: 21 },
    RawHuffmanCode { symbol_id: 218, code: 0x3ffffe8, bits: 26 },
    RawHuffmanCode { symbol_id: 219, code: 0x3ffffe9, bits: 26 },
    RawHuffmanCode { symbol_id: 220, code: 0xffffffd, bits: 28 },
    RawHuffmanCode { symbol_id: 221, code: 0x7ffffe3, bits: 27 },
    RawHuffmanCode { symbol_id: 222, code: 0x7ffffe4, bits: 27 },
    RawHuffmanCode { symbol_id: 223, code: 0x7ffffe5, bits: 27 },
    RawHuffmanCode { symbol_id: 224, code: 0xfffec, bits: 20 },
    RawHuffmanCode { symbol_id: 225, code: 0xfffff3, bits: 24 },
    RawHuffmanCode { symbol_id: 226, code: 0xfffed, bits: 20 },
    RawHuffmanCode { symbol_id: 227, code: 0x1fffe6, bits: 21 },
    RawHuffmanCode { symbol_id: 228, code: 0x3fffe9, bits: 22 },
    RawHuffmanCode { symbol_id: 229, code: 0x1fffe7, bits: 21 },
    RawHuffmanCode { symbol_id: 230, code: 0x1fffe8, bits: 21 },
    RawHuffmanCode { symbol_id: 231, code: 0x7ffff3, bits: 23 },
    RawHuffmanCode { symbol_id: 232, code: 0x3fffea, bits: 22 },
    RawHuffmanCode { symbol_id: 233, code: 0x3fffeb, bits: 22 },
    RawHuffmanCode { symbol_id: 234, code: 0x1ffffee, bits: 25 },
    RawHuffmanCode { symbol_id: 235, code: 0x1ffffef, bits: 25 },
    RawHuffmanCode { symbol_id: 236, code: 0xfffff4, bits: 24 },
    RawHuffmanCode { symbol_id: 237, code: 0xfffff5, bits: 24 },
    RawHuffmanCode { symbol_id: 238, code: 0x3ffffea, bits: 26 },
    RawHuffmanCode { symbol_id: 239, code: 0x7ffff4, bits: 23 },
    RawHuffmanCode { symbol_id: 240, code: 0x3ffffeb, bits: 26 },
    RawHuffmanCode { symbol_id: 241, code: 0x7ffffe6, bits: 27 },
    RawHuffmanCode { symbol_id: 242, code: 0x3ffffec, bits: 26 },
    RawHuffmanCode { symbol_id: 243, code: 0x3ffffed, bits: 26 },
    RawHuffmanCode { symbol_id: 244, code: 0x7ffffe7, bits: 27 },
    RawHuffmanCode { symbol_id: 245, code: 0x7ffffe8, bits: 27 },
    RawHuffmanCode { symbol_id: 246, code: 0x7ffffe9, bits: 27 },
    RawHuffmanCode { symbol_id: 247, code: 0x7ffffea, bits: 27 },
    RawHuffmanCode { symbol_id: 248, code: 0x7ffffeb, bits: 27 },
    RawHuffmanCode { symbol_id: 249, code: 0xfffffffe, bits: 28 },
    RawHuffmanCode { symbol_id: 250, code: 0x7ffffec, bits: 27 },
    RawHuffmanCode { symbol_id: 251, code: 0x7ffffed, bits: 27 },
    RawHuffmanCode { symbol_id: 252, code: 0x7ffffee, bits: 27 },
    RawHuffmanCode { symbol_id: 253, code: 0x7ffffef, bits: 27 },
    RawHuffmanCode { symbol_id: 254, code: 0x7fffff0, bits: 27 },
    RawHuffmanCode { symbol_id: 255, code: 0x3ffffee, bits: 26 },
    RawHuffmanCode { symbol_id: 256, code: 0x3fffffff, bits: 30 }, 
];

// Constants for LUT generation and FSM states
pub const K_LOOKUP_BITS: usize = 8;
pub const MAX_SYMBOLS_PER_LUT_ENTRY: usize = 2; 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LutEntry {
    pub symbols: [u8; MAX_SYMBOLS_PER_LUT_ENTRY],
    pub num_decoded: u8,
    pub bits_consumed: u8,
    pub next_decoder_state_id: u16,
}

impl Default for LutEntry {
    fn default() -> Self {
        LutEntry {
            symbols: [0; MAX_SYMBOLS_PER_LUT_ENTRY], 
            num_decoded: 0,
            bits_consumed: 0,
            next_decoder_state_id: STATE_ERROR, 
        }
    }
}

pub const STATE_CONTINUE_LUT: u16 = 0;
pub const STATE_EOS_DECODED: u16 = 0xFFFE;
pub const STATE_ERROR: u16 = 0xFFFF;

// Placeholder for the actual LUT which will be generated by build.rs or manually


pub const DECODING_LUT: [LutEntry; 1 << K_LOOKUP_BITS] = [LutEntry::default(); 1 << K_LOOKUP_BITS];
