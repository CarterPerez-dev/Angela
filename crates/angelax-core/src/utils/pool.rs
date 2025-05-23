// This module provides high-performance object and buffer pools for reducing allocations in hot paths.
// It uses lock-free data structures and RAII guards to ensure safe, efficient resource management across the framework.

use super::atomic::{AtomicBitmap, AtomicCounter};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::mem::MaybeUninit;
use std::ptr;

/// Pooled object wrapper that returns to pool on drop
pub struct PooledObject<T> {
    value: Option<T>,
    pool: Arc<ObjectPool<T>>,
    index: usize,
}

impl<T> Deref for PooledObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value.as_ref().unwrap()
    }
}

impl<T> DerefMut for PooledObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_mut().unwrap()
    }
}

impl<T> Drop for PooledObject<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.take() {
            self.pool.return_object(self.index, value);
        }
    }
}

/// Lock-free object pool
pub struct ObjectPool<T> {
    objects: Vec<MaybeUninit<T>>,
    available: AtomicBitmap,
    factory: Box<dyn Fn() -> T + Send + Sync>,
    reset: Option<Box<dyn Fn(&mut T) + Send + Sync>>,
    metrics: PoolMetrics,
}

impl<T> ObjectPool<T> {
    /// I'm creating a new object pool with the specified capacity
    pub fn new<F>(capacity: usize, factory: F) -> Arc<Self>
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        let mut objects = Vec::with_capacity(capacity);
        let available = AtomicBitmap::new(capacity);

        // Pre-allocate all objects
        for i in 0..capacity {
            objects.push(MaybeUninit::new(factory()));
            available.set(i);
        }

        Arc::new(Self {
            objects,
            available,
            factory: Box::new(factory),
            reset: None,
            metrics: PoolMetrics::new(),
        })
    }

    /// Create pool with reset function
    pub fn with_reset<F, R>(capacity: usize, factory: F, reset: R) -> Arc<Self>
    where
        F: Fn() -> T + Send + Sync + 'static,
        R: Fn(&mut T) + Send + Sync + 'static,
    {
        let mut pool = Self::new(capacity, factory);
        Arc::get_mut(&mut pool).unwrap().reset = Some(Box::new(reset));
        pool
    }

    /// Get an object from the pool
    pub fn get(self: &Arc<Self>) -> Option<PooledObject<T>> {
        self.metrics.requests.fetch_add(1);

        if let Some(index) = self.available.find_and_set() {
            self.metrics.hits.fetch_add(1);
            
            let value = unsafe {
                ptr::read(self.objects[index].as_ptr())
            };

            Some(PooledObject {
                value: Some(value),
                pool: self.clone(),
                index,
            })
        } else {
            self.metrics.misses.fetch_add(1);
            None
        }
    }

    /// Get or create an object
    pub fn get_or_create(self: &Arc<Self>) -> PooledObject<T> {
        self.get().unwrap_or_else(|| {
            // Pool exhausted, create a new object
            // This won't be returned to the pool
            PooledObject {
                value: Some((self.factory)()),
                pool: self.clone(),
                index: usize::MAX, // Sentinel value
            }
        })
    }

    /// Return an object to the pool
    fn return_object(&self, index: usize, mut value: T) {
        if index == usize::MAX {
            // Object was created outside pool, just drop it
            return;
        }

        self.metrics.returns.fetch_add(1);

        // Reset object if reset function provided
        if let Some(reset) = &self.reset {
            reset(&mut value);
        }

        unsafe {
            ptr::write(self.objects[index].as_mut_ptr(), value);
        }

        self.available.clear(index);
    }

    /// Get pool metrics
    pub fn metrics(&self) -> &PoolMetrics {
        &self.metrics
    }
}

unsafe impl<T: Send> Send for ObjectPool<T> {}
unsafe impl<T: Send> Sync for ObjectPool<T> {}

/// Buffer pool for reusable byte buffers
pub struct BufferPool {
    small_pool: Arc<ObjectPool<Vec<u8>>>,  // 4KB buffers
    medium_pool: Arc<ObjectPool<Vec<u8>>>, // 64KB buffers
    large_pool: Arc<ObjectPool<Vec<u8>>>,  // 1MB buffers
}

impl BufferPool {
    /// I'm creating a tiered buffer pool
    pub fn new() -> Self {
        Self {
            small_pool: ObjectPool::with_reset(
                256,
                || Vec::with_capacity(4096),
                |v| v.clear(),
            ),
            medium_pool: ObjectPool::with_reset(
                64,
                || Vec::with_capacity(65536),
                |v| v.clear(),
            ),
            large_pool: ObjectPool::with_reset(
                16,
                || Vec::with_capacity(1048576),
                |v| v.clear(),
            ),
        }
    }

    /// Get a buffer of at least the specified size
    pub fn get(&self, min_size: usize) -> PooledObject<Vec<u8>> {
        if min_size <= 4096 {
            self.small_pool.get_or_create()
        } else if min_size <= 65536 {
            self.medium_pool.get_or_create()
        } else {
            self.large_pool.get_or_create()
        }
    }

    /// Get buffer pool metrics
    pub fn metrics(&self) -> BufferPoolMetrics {
        BufferPoolMetrics {
            small: self.small_pool.metrics().clone(),
            medium: self.medium_pool.metrics().clone(),
            large: self.large_pool.metrics().clone(),
        }
    }
}

impl Default for BufferPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Pool metrics for monitoring
#[derive(Debug)]
pub struct PoolMetrics {
    pub requests: AtomicCounter,
    pub hits: AtomicCounter,
    pub misses: AtomicCounter,
    pub returns: AtomicCounter,
}

impl PoolMetrics {
    fn new() -> Self {
        Self {
            requests: AtomicCounter::new(0),
            hits: AtomicCounter::new(0),
            misses: AtomicCounter::new(0),
            returns: AtomicCounter::new(0),
        }
    }

    /// Calculate hit rate
    pub fn hit_rate(&self) -> f64 {
        let requests = self.requests.load();
        if requests == 0 {
            return 0.0;
        }
        self.hits.load() as f64 / requests as f64
    }
}

impl Clone for PoolMetrics {
    fn clone(&self) -> Self {
        Self {
            requests: AtomicCounter::new(self.requests.load()),
            hits: AtomicCounter::new(self.hits.load()),
            misses: AtomicCounter::new(self.misses.load()),
            returns: AtomicCounter::new(self.returns.load()),
        }
    }
}

/// Buffer pool metrics
#[derive(Debug)]
pub struct BufferPoolMetrics {
    pub small: PoolMetrics,
    pub medium: PoolMetrics,
    pub large: PoolMetrics,
}

/// Specialized pool for HTTP request/response objects
pub struct HttpObjectPool {
    pub request_pool: Arc<ObjectPool<HttpRequestBuffer>>,
    pub response_pool: Arc<ObjectPool<HttpResponseBuffer>>,
    pub header_pool: Arc<ObjectPool<Vec<(String, String)>>>,
}

/// Pre-allocated HTTP request buffer
pub struct HttpRequestBuffer {
    pub method: String,
    pub uri: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl HttpRequestBuffer {
    fn new() -> Self {
        Self {
            method: String::with_capacity(16),
            uri: String::with_capacity(256),
            headers: Vec::with_capacity(32),
            body: Vec::with_capacity(8192),
        }
    }

    fn reset(&mut self) {
        self.method.clear();
        self.uri.clear();
        self.headers.clear();
        self.body.clear();
    }
}

/// Pre-allocated HTTP response buffer
pub struct HttpResponseBuffer {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl HttpResponseBuffer {
    fn new() -> Self {
        Self {
            status: 200,
            headers: Vec::with_capacity(16),
            body: Vec::with_capacity(8192),
        }
    }

    fn reset(&mut self) {
        self.status = 200;
        self.headers.clear();
        self.body.clear();
    }
}

impl HttpObjectPool {
    /// I'm creating specialized pools for HTTP objects
    pub fn new() -> Self {
        Self {
            request_pool: ObjectPool::with_reset(
                128,
                HttpRequestBuffer::new,
                HttpRequestBuffer::reset,
            ),
            response_pool: ObjectPool::with_reset(
                128,
                HttpResponseBuffer::new,
                HttpResponseBuffer::reset,
            ),
            header_pool: ObjectPool::with_reset(
                256,
                || Vec::with_capacity(32),
                |v| v.clear(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::sync::Barrier;

    #[test]
    fn test_object_pool() {
        let pool = ObjectPool::new(3, || vec![0u8; 100]);
        
        // Get objects
        let mut obj1 = pool.get().unwrap();
        let mut obj2 = pool.get().unwrap();
        let mut obj3 = pool.get().unwrap();
        
        // Pool should be exhausted
        assert!(pool.get().is_none());
        
        // Modify objects
        obj1[0] = 1;
        obj2[0] = 2;
        obj3[0] = 3;
        
        // Return one object
        drop(obj1);
        
        // Should be able to get one more
        let obj4 = pool.get().unwrap();
        assert_eq!(obj4[0], 1); // Got the same object back
    }

    #[test]
    fn test_object_pool_with_reset() {
        let pool = ObjectPool::with_reset(
            2,
            || vec![0u8; 10],
            |v| v.fill(0),
        );
        
        let mut obj = pool.get().unwrap();
        obj.fill(42);
        drop(obj);
        
        let obj = pool.get().unwrap();
        assert!(obj.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_buffer_pool() {
        let pool = BufferPool::new();
        
        let small = pool.get(1000);
        assert!(small.capacity() >= 4096);
        
        let medium = pool.get(50000);
        assert!(medium.capacity() >= 65536);
        
        let large = pool.get(1000000);
        assert!(large.capacity() >= 1048576);
    }

    #[test]
    fn test_pool_metrics() {
        let pool = ObjectPool::new(2, || 42);
        
        let _obj1 = pool.get();
        let _obj2 = pool.get();
        let _obj3 = pool.get(); // Will fail
        
        let metrics = pool.metrics();
        assert_eq!(metrics.requests.load(), 3);
        assert_eq!(metrics.hits.load(), 2);
        assert_eq!(metrics.misses.load(), 1);
        assert_eq!(metrics.hit_rate(), 2.0 / 3.0);
    }

    #[test]
    fn test_concurrent_pool_access() {
        let pool = ObjectPool::new(10, || vec![0u8; 100]);
        let barrier = Arc::new(Barrier::new(5));
        let mut handles = vec![];
        
        for i in 0..5 {
            let pool = pool.clone();
            let barrier = barrier.clone();
            
            handles.push(thread::spawn(move || {
                barrier.wait();
                
                for _ in 0..20 {
                    if let Some(mut obj) = pool.get() {
                        obj[0] = i;
                        thread::yield_now();
                    }
                }
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // All objects should have been returned
        let metrics = pool.metrics();
        assert!(metrics.returns.load() > 0);
    }

    #[test]
    fn test_http_object_pool() {
        let pool = HttpObjectPool::new();
        
        let mut req = pool.request_pool.get().unwrap();
        req.method = "GET".to_string();
        req.uri = "/test".to_string();
        req.headers.push(("Host".to_string(), "example.com".to_string()));
        
        drop(req);
        
        // Get it back and verify it was reset
        let req = pool.request_pool.get().unwrap();
        assert!(req.method.is_empty());
        assert!(req.uri.is_empty());
        assert!(req.headers.is_empty());
    }
}
