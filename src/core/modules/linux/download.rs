use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use std::fs::File;
use std::io::Write;

pub struct DownloadLinux;

impl Module for DownloadLinux {
    fn name(&self) -> &'static str { "linux/download" }
    fn description(&self) -> &'static str { "Download any file from target (Linux) using base64" }
    fn usage(&self) -> &'static str { "Usage: run-module linux/upload <session_id> <remote_path>" }
    fn platform(&self) -> &'static str { "linux" }
    fn category(&self) -> &'static str { "file-transfer" }
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, args: Vec<String>) {
        let file_path = match args.get(0) {
            Some(path) => path,
            None => {
                LogHandler::error("No file provided for download");
                return;
            }
        };

        if let Some(session) = session_manager.get(session_id) {
            let mut locked = session.stream.lock().unwrap();
            let marker = "__AEGIS_END__";
            let cmd = format!("base64 \"{}\" ; echo {}\n", file_path, marker);
            match crate::cli::output::send_command_and_get_output_until(&mut locked, &cmd, marker) {
                Ok(data) => {
                    let marker_bytes = marker.as_bytes();
                    if let Some(idx) = data.windows(marker_bytes.len()).rposition(|window| window == marker_bytes) {
                        let file_content = &data[..idx];
                        // Skip the echoed command
                        let first_newline = file_content.iter().position(|&b| b == b'\n' || b == b'\r');
                        let b64_bytes = match first_newline {
                            Some(pos) => &file_content[pos+1..],
                            None => file_content,
                        };
                        // Remove any trailing new lines
                        let b64_str = file_content
                            .split(|&b| b == b'\n' || b == b'\r')
                            .filter_map(|line| {
                                let line = std::str::from_utf8(line).ok()?.trim();
                                // Only keep lines that look base64
                                if !line.is_empty() && line.chars().all(|c| c.is_ascii_alphanumeric() || "+/=".contains(c)) {
                                    Some(line)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("");

                        match base64::decode(&b64_str) {
                            Ok(bin) => {
                                let filename = std::path::Path::new(file_path)
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("downloaded_file.bin");
                                if let Ok(mut f) = File::create(filename) {
                                    if let Err(e) = f.write_all(&bin) {
                                        LogHandler::error(&format!("Failed to write file: {}", e));
                                    } else {
                                        LogHandler::success(&format!("Downloaded '{}' to '{}'", file_path, filename));
                                    }
                                } else {
                                    LogHandler::error("Failed to create local file");
                                }
                            }
                            Err(e) => LogHandler::error(&format!("Failed to decode base64: {}", e)),
                        }
                    } else {
                        LogHandler::error("End marker not found in output");
                    }
                }
                Err(e) => LogHandler::error(&format!("Failed to download file: {}", e)),
            }
        }
    }
}