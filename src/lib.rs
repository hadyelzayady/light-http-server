pub mod common;
pub mod http_request;
pub mod http_response;
use std::{
    fmt::Debug,
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
#[derive(Debug)]
struct Worker {
    thread: thread::JoinHandle<()>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;
impl Worker {
    fn new(channel: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        Self {
            thread: thread::spawn(move || {
                let thread_id = thread::current().id();
                loop {
                    let job = channel.lock().unwrap().recv().unwrap();
                    println!("Worker with thread {:?} got a job; executing.", thread_id);
                    job();
                }
            }),
        }
    }
}
#[derive(Debug)]
pub enum PoolCreationError {
    SizeError,
}
impl ThreadPool {
    fn new(size: usize) -> Self {
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for _ in 0..size {
            workers.push(Worker::new(receiver.clone()));
        }
        Self { workers, sender }
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size < 1 {
            return Err(PoolCreationError::SizeError);
        }
        Ok(Self::new(size))
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}
