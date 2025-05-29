use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use crate::cli::output::send_command;

use std::io::Write;

pub struct InteractiveShellLinux;

impl Module for InteractiveShellLinux {
    fn name(&self) -> &'static str { "linux/interactive-shell" }
    fn description(&self) -> &'static str { "Upgrade to a fully interactive TTY shell" }
    fn usage(&self) -> &'static str { "Usage: run-module linux/interactive-shell <session_id>" }
    fn platform(&self) -> &'static str { "linux" }
    fn category(&self) -> &'static str { "session" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, _args: Vec<String>) {
        if let Some(session) = session_manager.get(session_id) {
            let cmds = [
                "python3 -c 'import pty; pty.spawn(\"/bin/bash\")'\n",
                "export TERM=xterm\n",
                "export shell=/bin/bash\n",
                "stty rows 40 columns 120\n",
                "reset\n"
            ];
            for cmd in cmds.iter() {
                if let Err(e) = send_command(&session, cmd) {
                    LogHandler::error(&format!("Failed to send command: {}", e));
                    return;
                }
            }
            LogHandler::success("Sent interactive shell upgrade commands.");
        }
    }
}