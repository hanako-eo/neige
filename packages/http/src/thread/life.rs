use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
pub enum WorkerLifeState {
    Life,
    Die,
}

pub struct WorkerLife(Arc<Mutex<WorkerLifeState>>);

unsafe impl Send for WorkerLife {}
unsafe impl Sync for WorkerLife {}

impl WorkerLife {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(WorkerLifeState::Life)))
    }

    pub fn get(&self) -> WorkerLifeState {
        let state = self.0.lock().unwrap();
        *state
    }

    pub fn die(&mut self) {
        let mut state = self.0.lock().unwrap();
        *state = WorkerLifeState::Die;
    }

    pub fn is_die(&self) -> bool {
        matches!(self.get(), WorkerLifeState::Die)
    }
}

impl Clone for WorkerLife {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
