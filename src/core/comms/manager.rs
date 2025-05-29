use std::sync::{Arc, Mutex};

use super::Comms;

pub struct CommsManager {
    listeners: Arc<Mutex<Vec<std::thread::JoinHandle<()>>>>,
}

impl CommsManager {
    pub fn new() -> Self {
        Self { listeners: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn add_comms(&mut self, mut comms: Box<dyn Comms + Send>) {
        let handle = std::thread::spawn(move || {
            comms.start();
        });

        self.listeners.lock().unwrap().push(handle);
    }
}