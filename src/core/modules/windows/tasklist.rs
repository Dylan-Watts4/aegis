use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;

pub struct TasklistWindows;

impl Module for TasklistWindows {
    fn name(&self) -> &'static str { "windows/tasklist" }
    fn description(&self) -> &'static str { "List running processes (tasklist)" }
    fn usage(&self) -> &'static str { "Usage: run-module windows/tasklist <session_id>" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            let mut locked = session.stream.lock().unwrap();
            let marker = "__AEGIS_END__";
            let cmd = format!("tasklist ; echo {}\n", marker);
            match crate::cli::output::send_command_and_get_output_until(&mut *locked, &cmd, marker) {
                Ok(data) => {
                    if let Some(idx) = data.windows(marker.len()).rposition(|window| window == marker.as_bytes()) {
                        let output = &data[..idx];
                        print!("{}", String::from_utf8_lossy(output));
                    } else {
                        LogHandler::error("End marker not found in output");
                    }
                }
                Err(e) => LogHandler::error(&format!("Failed to run tasklist: {}", e)),
            }
        }
    }
}