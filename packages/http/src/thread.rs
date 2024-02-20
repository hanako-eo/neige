use std::sync::mpsc;
use std::sync::{Arc, Mutex};

pub mod life;
pub mod worker;

pub(super) type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<worker::Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(pool: u32) -> Self {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(pool as usize);

        // TODO: auto-healing system
        for _ in 0..pool {
            workers.push(worker::Worker::new(Arc::clone(&receiver)))
        }

        Self { workers, sender }
    }

    pub fn heal(&mut self) {
        for worker in &mut self.workers {
            worker.spawn();
        }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            worker.kill();
        }
    }
}
