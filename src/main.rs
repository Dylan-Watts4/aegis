mod cli;
mod core;

use std::sync::{Arc, Mutex};
use crate::core::session::SessionManager;
use crate::core::comms::manager::CommsManager;

fn main() {
   println!("Starting Aegis...");
   let session_manager = Arc::new(SessionManager::new());
   let comms_manager = Arc::new(Mutex::new(CommsManager::new()));

   cli::start_repl(session_manager.clone(), comms_manager.clone());
}