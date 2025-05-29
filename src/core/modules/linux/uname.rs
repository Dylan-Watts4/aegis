use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;

pub struct UnameLinux;

impl Module for UnameLinux {
    fn name(&self) -> &'static str { "linux/uname" }
    fn description(&self) -> &'static str { "Get linux system information (uname -a)" }
    fn usage(&self) -> &'static str { "Usage: run-module linux/uname <session_id>" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, _args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            let mut locked = session.stream.lock().unwrap();
            if let Err(e) = crate::cli::send_command_and_print_output(&mut *locked, "uname -a\n") {
                LogHandler::error(&format!("Failed to run uname: {}", e));
            }
        }
    }
}