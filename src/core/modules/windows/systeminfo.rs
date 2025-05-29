use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;

pub struct SysteminfoWindows;

impl Module for SysteminfoWindows {
    fn name(&self) -> &'static str { "windows/systeminfo" }
    fn description(&self) -> &'static str { "Get Windows system information (systeminfo)" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, _args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            let mut locked = session.stream.lock().unwrap();
            if let Err(e) = crate::cli::send_command_and_print_output(&mut *locked, "systeminfo\n") {
                LogHandler::error(&format!("Failed to run systeminfo: {}", e));
            }
        }
    }
}