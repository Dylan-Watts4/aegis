use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use crate::core::comms::Comms;
use crate::core::session::SessionManager;
use crate::core::loghandler::LogHandler;

pub struct TcpComms {
    pub port: u16,
    pub session_manager: Arc<SessionManager>,
}

impl TcpComms {
    pub fn new(port: u16, session_manager: Arc<SessionManager>) -> Self {
        TcpComms { port, session_manager }
    }

    pub fn connect_to_bind_shell(&self, ip: &str, port: u16) {
        match TcpStream::connect((ip, port)) {
            Ok(stream) => {
                let session_id = self.session_manager.add_tcp_session(stream.try_clone().unwrap());
                LogHandler::success(&format!("[+] New TCP session: {}", session_id));
            }
            Err(e) => LogHandler::error(&format!("[!] Connection error: {}", e)),
        }
    }
}

impl Comms for TcpComms {
    fn start(&self) {
        let port = self.port;
        let session_manager = Arc::clone(&self.session_manager);

        std::thread::spawn(move || {
            let listener = match TcpListener::bind(("0.0.0.0", port)) {
                Ok(l) => l,
                Err(e) => {
                    LogHandler::error(&format!("Failed to bind TCP port {}: {}", port, e));
                    return;
                }
            };

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let session_manager = Arc::clone(&session_manager);
                        let session_id = session_manager.add_tcp_session(stream.try_clone().unwrap());
                        LogHandler::success(&format!("[+] New TCP session: {}", session_id));
                    }
                    Err(e) => LogHandler::error(&format!("[!] Connection error: {}", e)),
                }
            }
        });
    }

    fn send(&self, session_id: usize, data: &[u8]) -> Result<(), String> {
        if let Some(session) = self.session_manager.get(session_id) {
            match &session.stream {
                crate::core::session::ProtocolStream::Tcp(stream) => {
                    let mut stream = stream.lock().map_err(|e| e.to_string())?;
                    stream.write_all(data).map_err(|e| e.to_string())?;
                }
                crate::core::session::ProtocolStream::Udp { socket, remote_addr } => {
                    socket.send_to(data, remote_addr).map_err(|e| e.to_string())?;
                }
            }
            if let Ok(mut last_active) = session.last_active.lock() {
                *last_active = std::time::SystemTime::now();
            }
            Ok(())
        } else {
            Err("Invalid session".into())
        }
    }


    fn receive(&self, session_id: usize) -> Result<Vec<u8>, String> {
        if let Some(session) = self.session_manager.get(session_id) {
            let mut buf = [0u8; 4096];
            let size = match &session.stream {
                crate::core::session::ProtocolStream::Tcp(stream) => {
                    let mut stream = stream.lock().map_err(|e| e.to_string())?;
                    stream.read(&mut buf).map_err(|e| e.to_string())?
                }
                crate::core::session::ProtocolStream::Udp { socket, .. } => {
                    socket.recv(&mut buf).map_err(|e| e.to_string())?
                }
            };
            if let Ok(mut last_active) = session.last_active.lock() {
                *last_active = std::time::SystemTime::now();
            }
            Ok(buf[..size].to_vec())
        } else {
            Err("Invalid session".into())
        }
    }
}