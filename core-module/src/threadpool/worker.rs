#[warn(unused_imports)]
use std::thread;
use std::sync::{mpsc, Arc, Mutex};


/// Represents a worker in the threadpool.
pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}


impl Worker {
    /// Creates a new worker
    ///
    /// # Args
    /// - `id`: The worker's unique identifier
    /// `receiver`: Shared receiver from which the worker retrieves the jobs.
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<super::Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // Lock receiver and wait for jobs
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    log::info!("Worker {id} got a job; executing....");
                    job();
                }
                Err(_) => {
                    log::info!("Worker {id} disconnected; shutting down...");
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

