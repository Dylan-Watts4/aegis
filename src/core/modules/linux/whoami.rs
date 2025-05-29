use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;

pub struct WhoamiLinux;

impl Module for WhoamiLinux {
    fn name(&self) -> &'static str { "linux/whoami" }
    fn description(&self) -> &'static str { "Get current user (Linux)" }
    fn usage(&self) -> &'static str { "Usage: run-module <session_id>" }
    fn platform(&self) -> &'static str { "linux" }
    fn category(&self) -> &'static str { "enumeration" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, _args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            let mut locked = session.stream.lock().unwrap();
            if let Err(e) = crate::cli::send_command_and_print_output(&mut *locked, "whoami\n") {
                LogHandler::error(&format!("Failed to run whoami: {}", e));
            }
        }
    }
}