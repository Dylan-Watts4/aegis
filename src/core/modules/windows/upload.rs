use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use crate::cli::output::{send_command_and_print_output, send_chunks};
use std::fs;
use base64;

pub struct UploadWindows;

impl Module for UploadWindows {
    fn name(&self) -> &'static str { "windows/upload" }
    fn description(&self) -> &'static str { "Upload a file to the target (Windows) using base64" }
    fn usage(&self) -> &'static str { "Usage: run-module windows/upload <session_id> <local_path> <remote_path>" }
    fn platform(&self) -> &'static str { "windows" }
    fn category(&self) -> &'static str { "file-transfer" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, args: Vec<String>) {
        let (local_path, remote_path) = match (args.get(0), args.get(1)) {
            (Some(l), Some(r)) => (l, r),
            _ => {
                LogHandler::error("Usage: windows/upload <local_path> <remote_path>");
                return;
            }
        };

        let file_data = match fs::read(local_path) {
            Ok(data) => data,
            Err(e) => {
                LogHandler::error(&format!("Failed to read local file: {}", e));
                return;
            }
        };
        let b64 = base64::encode(&file_data);

        if let Some(session) = session_manager.get(session_id) {
            let tmp_path = "C:\\Windows\\Temp\\aegis_upload.b64";
            // Remove any old tmp file
            let _ = send_command_and_print_output(&session, &format!("del {}\n", tmp_path));
            // Send b64 in chunks
            let _ = send_chunks(&session, b64.as_bytes(), 512, 
                &format!("powershell -Command \"Add-Content -Path '{}' -Value '", tmp_path),
                "'\"\n"
            );
            // Decode on the remote side
            let decode_cmd = format!("certutil -decode {} \"{}\" ; del {} \n", tmp_path, remote_path, tmp_path);
            let _ = send_command_and_print_output(&session, &decode_cmd);
            LogHandler::success(&format!("Uploaded '{}' to '{}'", local_path, remote_path));
        }
    }
}