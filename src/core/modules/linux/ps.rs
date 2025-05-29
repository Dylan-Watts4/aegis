use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use crate::cli::output::send_command_and_get_output_until;

pub struct PsLinux;

impl Module for PsLinux {
    fn name(&self) -> &'static str { "linux/ps" }
    fn description(&self) -> &'static str { "List running processes (ps aux)" }
    fn usage(&self) -> &'static str { "Usage: run-module linux/ps <session_id>" }
    fn platform(&self) -> &'static str { "linux" }
    fn category(&self) -> &'static str { "enumeration" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            let marker = "__AEGIS_END__";
            let cmd = format!("ps aux; echo {}\n", marker);
            match send_command_and_get_output_until(&session, &cmd, marker) {
                Ok(data) => {
                    if let Some(idx) = data.windows(marker.len()).rposition(|window| window == marker.as_bytes()) {
                        let output = &data[..idx];
                        print!("{}", String::from_utf8_lossy(output));
                    } else {
                        LogHandler::error("End marker not found in output.");
                    }
                }
                Err(e) => LogHandler::error(&format!("Failed to run ps: {}", e)),
            } 
        }
    }
}