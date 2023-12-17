use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

use crate::owner::Owner;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(pool: u32) -> Self {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(pool as usize);

        for id in 0..pool {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        Self {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    // never call actually
    fn drop(&mut self) {
        for worker in &mut self.workers {
            if let Owner::Control(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: u32,
    thread: Owner<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let Ok(job) = receiver.lock().unwrap().recv() else {
                break;
            };

            job();
        });

        Worker {
            id,
            thread: Owner::Control(thread)
        }
    }
}
