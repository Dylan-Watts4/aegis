use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use crate::cli::output::send_command_and_print_output;

pub struct SysteminfoWindows;

impl Module for SysteminfoWindows {
    fn name(&self) -> &'static str { "windows/systeminfo" }
    fn description(&self) -> &'static str { "Get Windows system information (systeminfo)" }
    fn usage(&self) -> &'static str { "Usage: run-module windows/systeminfo <session_id>" }
    fn platform(&self) -> &'static str { "windows" }
    fn category(&self) -> &'static str { "enumeration" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, _args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            if let Err(e) = send_command_and_print_output(&session, "systeminfo\n") {
                LogHandler::error(&format!("Failed to run systeminfo: {}", e));
            }
        }
    }
}