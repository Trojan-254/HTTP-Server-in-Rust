use std::sync::{mpsc, Arc, Mutex};
use log::info;
use std::thread::JoinHandle;

mod worker;
use worker::Worker;


/// A threadpool for managing and executing tasks in parallel
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

/// Type alias for a job that can be executed by a threadpool.
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Creates a new `Threadpool`
    ///
    /// # Args
    /// - `size`: The number of threads in the pool.
    /// 
    /// # panics
    /// Panics if `size` is 0
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "Threadpool must be greater than 0");


        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();
        //let mut workers = Vec::with_capacity(size);
        //for id in 0..size {
        //    workers.push(Worker::new(id, Arc::clone(&receiver)));
        //}
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    


    /// Submits a job to the thread pool for execution.
    ///
    /// # Arguments
    /// - `f`: A closure that represents the job to be executed.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(sender) = &self.sender {
           if sender.send(Box::new(f)).is_err() {
               log::error!("Failed to send job to worker threads!.");
           } else {
               log::error!("Threadpool has already been shut down.");
           }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
       log::info!("Shutting down ThreadPool....");

       // Drop the sender to signal worker shutdown.
       self.sender.take();

       // Join all worker threads.
       for worker in &mut self.workers {
           log::info!("Shutting down worker {}...", worker.id);
           if let Some(thread) = worker.thread.take() {
              if thread.join().is_err() {
                 log::warn!("Worker {} failed to shut down cleanly.", worker.id);
               } else {
                 log::info!("Worker {} shut down succesfully.", worker.id);
               }
           }
       }
    }
}


/// Spawns a new thread outside the thread pool.
///
/// # Arguments
/// - `f`: A closure to execute in the thread.
///
/// # Returns
/// A handle to the newly spawned thread.
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
{
    std::thread::spawn(f)
}

