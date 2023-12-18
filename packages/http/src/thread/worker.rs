use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

use super::life::WorkerLife;

pub struct Worker {
    thread: Option<thread::JoinHandle<()>>,
    job_receiver: Arc<Mutex<mpsc::Receiver<super::Job>>>,

    life: WorkerLife,
}

impl Worker {
    pub(crate) fn new(receiver: Arc<Mutex<mpsc::Receiver<super::Job>>>) -> Self {
        let mut worker = Self {
            life: WorkerLife::new(),
            job_receiver: receiver,
            thread: None
        };
        worker.spawn();
        worker
    }

    pub(crate) fn spawn(&mut self) {
        if self.is_killed() || self.thread.is_some() && !self.thread.as_ref().unwrap().is_finished() {
            return;
        }

        let live = self.life.clone();
        let receiver = Arc::clone(&self.job_receiver);
        self.thread = Some(thread::spawn(move || loop {
            if live.is_die() {
                break;
            }

            let Ok(job) = receiver.lock().unwrap().recv() else {
                break;
            };

            job();
        }));
    }

    pub(crate) fn kill(&mut self) {
        self.life.die();
    }
    
    pub(crate) fn is_killed(&self) -> bool {
        self.life.is_die()
    }
}
