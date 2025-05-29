use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use crate::cli::output::send_command_and_print_output;

pub struct SudoCheckLinux;

impl Module for SudoCheckLinux {
    fn name(&self) -> &'static str { "linux/sudo-check" }
    fn description(&self) -> &'static str { "Check for sudo privileges and misconfigurations" }
    fn usage(&self) -> &'static str { "Usage: run-module linux/sudo-check <session_id>" }
    fn platform(&self) -> &'static str { "linux" }
    fn category(&self) -> &'static str { "privesc" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, _args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            let cmd = "sudo -l\n";
            if let Err(e) = send_command_and_print_output(&session, cmd) {
                LogHandler::error(&format!("Failed to run sudo check: {}", e));
            }
        }
    }
}