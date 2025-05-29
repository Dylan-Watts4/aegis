use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use std::fs;
use std::io::{self, Read};
use base64;

pub struct UploadLinux;

impl Module for UploadLinux {
    fn name(&self) -> &'static str { "linux/upload" }
    fn description(&self) -> &'static str { "Upload a file to the target (Linux) using base64" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, args: Vec<String>) {
        let (local_path, remote_path) = match (args.get(0), args.get(1)) {
            (Some(l), Some(r)) => (l, r),
            _ => {
                LogHandler::error("Usage: linux/upload <local_path> <remote_path>");
                return;
            }
        };

        // Read and base64-encode the local file
        let file_data = match fs::read(local_path) {
            Ok(data) => data,
            Err(e) => {
                LogHandler::error(&format!("Failed to read local file: {}", e));
                return;
            }
        };
        let b64 = base64::encode(&file_data);

        if let Some(session) = session_manager.get(session_id) {
            let mut locked = session.stream.lock().unwrap();
            let tmp_path = "/tmp/aegis_upload.b64";
            // Remove any old temp file
            let _ = crate::cli::output::send_command_and_print_output(&mut *locked, &format!("rm -f {}\n", tmp_path));
            // Send b64 data in chunks
            for chunk in b64.as_bytes().chunks(512) {
                let chunk_str = std::str::from_utf8(chunk).unwrap();
                let cmd = format!("echo '{}' >> {}\n", chunk_str, tmp_path);
                let _ = crate::cli::output::send_command_and_print_output(&mut *locked, &cmd);
            }
            // Decode on remote side
            let decode_cmd = format!("base64 -d {} > \"{}\" && rm -f {}\n", tmp_path, remote_path, tmp_path);
            let _ = crate::cli::output::send_command_and_print_output(&mut *locked, &decode_cmd);
            LogHandler::success(&format!("Uploaded '{}' to '{}'", local_path, remote_path));
        }
    }
}