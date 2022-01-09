use std::thread;

pub struct ThreadPool {
    //threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
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

        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send, // + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id:usize)->Self{
        let thread = thread::spawn(||{});
        Worker { id, thread }
    }
}
