use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use crate::cli::output::send_command_and_print_output;

pub struct WhoamiWindows;

impl Module for WhoamiWindows {
    fn name(&self) -> &'static str { "windows/whoami" }
    fn description(&self) -> &'static str { "Get current user (Windows)" }
    fn usage(&self) -> &'static str { "Usage: run-module windows/whoami <session_id>" }
    fn platform(&self) -> &'static str { "windows" }
    fn category(&self) -> &'static str { "enumeration" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, _args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            if let Err(e) = send_command_and_print_output(&session, "whoami\n") {
                LogHandler::error(&format!("Failed to run whoami: {}", e));
            }
        }
    }
}