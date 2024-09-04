use std::sync::{mpsc, Arc, Mutex};
use log::info;
use std::thread::JoinHandle;

mod worker;
use worker::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
       drop(self.sender.take());
       for worker in &mut self.workers {
           info!("Shutting down worker...");
           println!("Worker id: {}", worker.id);

           if let Some(thread) = worker.thread.take() {
              match thread.join() {
                 Ok(_) => info!("Worker {} shut down succesfully.", worker.id),
                 Err(e) => eprintln!("Worker {} failed to shut down: {:?}", worker.id, e),
              }
           }
       }
    }
}

pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
{
    std::thread::spawn(f)
}

