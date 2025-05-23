// This module provides lock-free atomic primitives optimized for web server workloads.
// It includes counters, bitmaps, and other concurrent data structures that minimize contention and maximize throughput in multi-threaded environments.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::cell::UnsafeCell;

/// Memory ordering for sequential consistency
pub use std::sync::atomic::Ordering::SeqCst;

/// High-performance atomic counter with cache-line padding
#[repr(align(64))] // Align to cache line to prevent false sharing
pub struct AtomicCounter {
    value: AtomicU64,
    _padding: [u8; 56], // Pad to full cache line
}

impl AtomicCounter {
    /// I'm creating a new atomic counter
    pub const fn new(initial: u64) -> Self {
        Self {
            value: AtomicU64::new(initial),
            _padding: [0; 56],
        }
    }

    /// Increment and return previous value
    #[inline]
    pub fn fetch_add(&self, val: u64) -> u64 {
        self.value.fetch_add(val, Ordering::Relaxed)
    }

    /// Decrement and return previous value
    #[inline]
    pub fn fetch_sub(&self, val: u64) -> u64 {
        self.value.fetch_sub(val, Ordering::Relaxed)
    }

    /// Get current value
    #[inline]
    pub fn load(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }

    /// Set value
    #[inline]
    pub fn store(&self, val: u64) {
        self.value.store(val, Ordering::Relaxed)
    }

    /// Compare and swap
    #[inline]
    pub fn compare_exchange(&self, current: u64, new: u64) -> Result<u64, u64> {
        self.value.compare_exchange(
            current,
            new,
            Ordering::AcqRel,
            Ordering::Acquire,
        )
    }
}

/// Atomic bitmap for tracking allocated resources
pub struct AtomicBitmap {
    bits: Vec<AtomicU64>,
    size: usize,
}

impl AtomicBitmap {
    /// Create a new bitmap with the given number of bits
    pub fn new(size: usize) -> Self {
        let num_words = (size + 63) / 64;
        let mut bits = Vec::with_capacity(num_words);
        
        for _ in 0..num_words {
            bits.push(AtomicU64::new(0));
        }
        
        Self { bits, size }
    }

    /// Set a bit atomically
    #[inline]
    pub fn set(&self, index: usize) -> bool {
        if index >= self.size {
            return false;
        }

        let word_index = index / 64;
        let bit_index = index % 64;
        let mask = 1u64 << bit_index;

        let prev = self.bits[word_index].fetch_or(mask, Ordering::AcqRel);
        (prev & mask) == 0
    }

    /// Clear a bit atomically
    #[inline]
    pub fn clear(&self, index: usize) -> bool {
        if index >= self.size {
            return false;
        }

        let word_index = index / 64;
        let bit_index = index % 64;
        let mask = 1u64 << bit_index;

        let prev = self.bits[word_index].fetch_and(!mask, Ordering::AcqRel);
        (prev & mask) != 0
    }

    /// Test if a bit is set
    #[inline]
    pub fn test(&self, index: usize) -> bool {
        if index >= self.size {
            return false;
        }

        let word_index = index / 64;
        let bit_index = index % 64;
        let mask = 1u64 << bit_index;

        (self.bits[word_index].load(Ordering::Acquire) & mask) != 0
    }

    /// Find first clear bit
    pub fn find_first_clear(&self) -> Option<usize> {
        for (word_idx, word) in self.bits.iter().enumerate() {
            let value = word.load(Ordering::Acquire);
            if value != u64::MAX {
                // Found a word with at least one clear bit
                let bit_idx = value.trailing_ones() as usize;
                let index = word_idx * 64 + bit_idx;
                if index < self.size {
                    return Some(index);
                }
            }
        }
        None
    }

    /// I'm finding and setting the first clear bit atomically
    pub fn find_and_set(&self) -> Option<usize> {
        for (word_idx, word) in self.bits.iter().enumerate() {
            let mut current = word.load(Ordering::Acquire);
            
            while current != u64::MAX {
                let bit_idx = current.trailing_ones() as usize;
                let new = current | (1u64 << bit_idx);
                
                match word.compare_exchange(current, new, Ordering::AcqRel, Ordering::Acquire) {
                    Ok(_) => {
                        let index = word_idx * 64 + bit_idx;
                        if index < self.size {
                            return Some(index);
                        }
                    }
                    Err(actual) => current = actual,
                }
            }
        }
        None
    }
}

/// Lock-free stack for object pooling
pub struct AtomicStack<T> {
    head: AtomicUsize,
    nodes: Vec<Node<T>>,
}

struct Node<T> {
    value: UnsafeCell<Option<T>>,
    next: AtomicUsize,
}

impl<T> AtomicStack<T> {
    /// Create a new atomic stack with pre-allocated nodes
    pub fn new(capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity);
        
        for i in 0..capacity {
            nodes.push(Node {
                value: UnsafeCell::new(None),
                next: AtomicUsize::new(i + 1),
            });
        }
        
        // Last node points to sentinel value
        if capacity > 0 {
            nodes[capacity - 1].next.store(usize::MAX, Ordering::Relaxed);
        }
        
        Self {
            head: AtomicUsize::new(0),
            nodes,
        }
    }

    /// Push a value onto the stack
    pub fn push(&self, value: T) -> bool {
        loop {
            let head = self.head.load(Ordering::Acquire);
            
            if head >= self.nodes.len() {
                return false; // Stack full
            }
            
            let node = &self.nodes[head];
            let next = node.next.load(Ordering::Relaxed);
            
            if self.head.compare_exchange(head, next, Ordering::Release, Ordering::Acquire).is_ok() {
                unsafe {
                    *node.value.get() = Some(value);
                }
                return true;
            }
        }
    }

    /// Pop a value from the stack
    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            
            if head == usize::MAX || head >= self.nodes.len() {
                return None; // Stack empty
            }
            
            let node = &self.nodes[head];
            let next = node.next.load(Ordering::Relaxed);
            
            if self.head.compare_exchange(head, next, Ordering::Release, Ordering::Acquire).is_ok() {
                unsafe {
                    let value = (*node.value.get()).take();
                    node.next.store(self.head.load(Ordering::Relaxed), Ordering::Relaxed);
                    return value;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_atomic_counter() {
        let counter = AtomicCounter::new(0);
        
        assert_eq!(counter.load(), 0);
        assert_eq!(counter.fetch_add(5), 0);
        assert_eq!(counter.load(), 5);
        assert_eq!(counter.fetch_sub(3), 5);
        assert_eq!(counter.load(), 2);
        
        assert_eq!(counter.compare_exchange(2, 10), Ok(2));
        assert_eq!(counter.load(), 10);
        assert_eq!(counter.compare_exchange(2, 20), Err(10));
    }

    #[test]
    fn test_atomic_bitmap() {
        let bitmap = AtomicBitmap::new(128);
        
        assert!(!bitmap.test(0));
        assert!(bitmap.set(0));
        assert!(bitmap.test(0));
        assert!(!bitmap.set(0)); // Already set
        
        assert!(bitmap.clear(0));
        assert!(!bitmap.test(0));
        assert!(!bitmap.clear(0)); // Already clear
        
        // Test find_first_clear
        bitmap.set(0);
        bitmap.set(1);
        assert_eq!(bitmap.find_first_clear(), Some(2));
        
        // Test find_and_set
        assert_eq!(bitmap.find_and_set(), Some(2));
        assert!(bitmap.test(2));
    }

    #[test]
    fn test_atomic_stack() {
        let stack = AtomicStack::new(10);
        
        assert!(stack.push(1));
        assert!(stack.push(2));
        assert!(stack.push(3));
        
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_concurrent_counter() {
        let counter = Arc::new(AtomicCounter::new(0));
        let mut handles = vec![];
        
        for _ in 0..10 {
            let counter_clone = counter.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    counter_clone.fetch_add(1);
                }
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(counter.load(), 10000);
    }

    #[test]
    fn test_concurrent_bitmap() {
        let bitmap = Arc::new(AtomicBitmap::new(1000));
        let mut handles = vec![];
        
        for _ in 0..10 {
            let bitmap_clone = bitmap.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..10 {
                    bitmap_clone.find_and_set();
                }
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Count set bits
        let mut count = 0;
        for i in 0..100 {
            if bitmap.test(i) {
                count += 1;
            }
        }
        assert_eq!(count, 100);
    }
}
