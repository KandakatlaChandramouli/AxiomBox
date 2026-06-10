use std::sync::{Arc, Mutex};

pub type CleanupFn = Box<dyn FnOnce() + Send + 'static>;

#[derive(Clone)]
pub struct CleanupManager {
    handlers: Arc<Mutex<Vec<CleanupFn>>>,
}

impl CleanupManager {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn register<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.handlers.lock().unwrap().push(Box::new(f));
    }
}

impl Drop for CleanupManager {
    fn drop(&mut self) {
        if let Ok(mut handlers) = self.handlers.lock() {
            while let Some(h) = handlers.pop() {
                h();
            }
        }
    }
}
