use std::{thread, sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>, // todo get rid of mpsc
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
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size{
            workers.push(Worker::new(id, 
                Arc::clone(&receiver)
            ));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id:usize, receiver:Arc<Mutex<Receiver<Job>>>)->Self{
        let thread = thread::spawn(move || loop { 
            let job = receiver
            .lock().unwrap()
            .recv().unwrap();    

            println!("Received a job {}", id);

            job();
        });
        Worker { id, thread }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
