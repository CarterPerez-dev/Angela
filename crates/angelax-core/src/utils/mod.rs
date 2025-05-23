// This module provides low-level utilities and optimizations used throughout the Angelax core.
// It includes SIMD operations, atomic primitives, and memory pool implementations that enable the framework's exceptional performance characteristics.

pub mod simd;
pub mod atomic;
pub mod pool;

// Re-export commonly used utilities
pub use simd::{
    SimdDelimiterFinder,
    SimdCrlfFinder,
    SimdUppercaseConverter,
    SimdTokenValidator,
    SimdWhitespaceSkipper,
};

pub use atomic::{
    AtomicCounter,
    AtomicBitmap,
    SeqCst,
};

pub use pool::{
    ObjectPool,
    BufferPool,
    PooledObject,
};

/// Cache line size for the target architecture
pub const CACHE_LINE_SIZE: usize = 64;

/// Align a value to cache line boundary
#[inline]
pub const fn align_to_cache_line(size: usize) -> usize {
    (size + CACHE_LINE_SIZE - 1) & !(CACHE_LINE_SIZE - 1)
}

/// Fast memory copy using SIMD when available
#[inline]
pub fn fast_copy(dst: &mut [u8], src: &[u8]) {
    debug_assert!(dst.len() >= src.len());
    
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    {
        use std::arch::x86_64::*;
        
        unsafe {
            let len = src.len();
            let mut offset = 0;
            
            // Copy 32 bytes at a time with AVX2
            while offset + 32 <= len {
                let chunk = _mm256_loadu_si256(src.as_ptr().add(offset) as *const __m256i);
                _mm256_storeu_si256(dst.as_mut_ptr().add(offset) as *mut __m256i, chunk);
                offset += 32;
            }
            
            // Copy remaining bytes
            dst[offset..].copy_from_slice(&src[offset..]);
        }
    }
    
    #[cfg(not(all(target_arch = "x86_64", target_feature = "avx2")))]
    {
        dst[..src.len()].copy_from_slice(src);
    }
}

/// Zero memory using SIMD when available
#[inline]
pub fn fast_zero(dst: &mut [u8]) {
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    {
        use std::arch::x86_64::*;
        
        unsafe {
            let len = dst.len();
            let mut offset = 0;
            let zero = _mm256_setzero_si256();
            
            // Zero 32 bytes at a time
            while offset + 32 <= len {
                _mm256_storeu_si256(dst.as_mut_ptr().add(offset) as *mut __m256i, zero);
                offset += 32;
            }
            
            // Zero remaining bytes
            for byte in &mut dst[offset..] {
                *byte = 0;
            }
        }
    }
    
    #[cfg(not(all(target_arch = "x86_64", target_feature = "avx2")))]
    {
        dst.fill(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_line_alignment() {
        assert_eq!(align_to_cache_line(0), 0);
        assert_eq!(align_to_cache_line(1), 64);
        assert_eq!(align_to_cache_line(64), 64);
        assert_eq!(align_to_cache_line(65), 128);
    }

    #[test]
    fn test_fast_copy() {
        let src = vec![1u8; 100];
        let mut dst = vec![0u8; 100];
        
        fast_copy(&mut dst, &src);
        assert_eq!(dst, src);
    }

    #[test]
    fn test_fast_zero() {
        let mut buffer = vec![0xFFu8; 100];
        fast_zero(&mut buffer);
        
        assert!(buffer.iter().all(|&b| b == 0));
    }
}
