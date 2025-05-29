use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use crate::cli::output::send_command_and_print_output;

pub struct UnameLinux;

impl Module for UnameLinux {
    fn name(&self) -> &'static str { "linux/uname" }
    fn description(&self) -> &'static str { "Get linux system information (uname -a)" }
    fn usage(&self) -> &'static str { "Usage: run-module linux/uname <session_id>" }
    fn platform(&self) -> &'static str { "linux" }
    fn category(&self) -> &'static str { "enumeration" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, _args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            if let Err(e) = send_command_and_print_output(&session, "uname -a\n") {
                LogHandler::error(&format!("Failed to run uname: {}", e));
            }
        }
    }
}