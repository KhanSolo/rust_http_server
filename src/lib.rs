pub struct ThreadPool {
    size: usize,
}

impl ThreadPool {
    /// Creates a new [`ThreadPool`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_http_server::ThreadPool;
    ///
    /// assert_eq!(ThreadPool::new(size), );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        ThreadPool { size }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
