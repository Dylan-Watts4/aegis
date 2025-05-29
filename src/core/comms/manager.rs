use std::sync::{Arc, Mutex};

use super::Comms;

pub struct CommsManager {
    comms: Arc<Mutex<Vec<Arc<dyn Comms + Send + Sync>>>>,
}

impl CommsManager {
    pub fn new() -> Self {
        Self { comms: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn add_comms(&mut self, mut comms: Arc<dyn Comms + Send + Sync>) {
        self.comms.lock().unwrap().push(comms);
    }
}