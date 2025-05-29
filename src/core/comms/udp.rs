use std::net::UdpSocket;
use std::sync::Arc;
use std::io;
use crate::core::session::SessionManager;
use crate::core::loghandler::LogHandler;
use crate::core::comms::Comms;

pub struct UdpComms {
    pub port: u16,
    pub session_manager: Arc<SessionManager>,
    pub socket: Arc<UdpSocket>,
}

impl UdpComms {
    pub fn new(port: u16, session_manager: Arc<SessionManager>) -> Self {
        let socket = Arc::new(UdpSocket::bind(("0.0.0.0", port)).expect("Failed to bind to UDP port"));
        UdpComms { port, session_manager, socket }
    }
}

impl Comms for UdpComms {
    fn start(&self) {
        let socket = Arc::clone(&self.socket);
        let session_manager = Arc::clone(&self.session_manager);

        std::thread::spawn(move || {
            let mut buf = [0u8; 4096];
            loop {
                match socket.recv_from(&mut buf) {
                    Ok((n, src)) => {
                        let session_id = session_manager.add_udp_session(socket.clone(), src);
                        LogHandler::success(&format!("[+] New UDP session: {} from {}", session_id, src));
                    }
                    Err(e) => {
                        LogHandler::error(&format!("UDP receive error: {}", e));
                        break;
                    }
                }
            }
        });
    }

    fn send(&self, session_id: usize, data: &[u8]) -> Result<(), String> {
        if let Some(session) = self.session_manager.get(session_id) {
            if let Some(addr) = session.remote_addr {
                self.socket.send_to(data, addr).map_err(|e| e.to_string())?;
                Ok(())
            } else {
                Err("Session has no remote address".into())
            }
        } else {
            Err("Invalid session".into())
        }
    }

    fn receive(&self, _session_id: usize) -> Result<Vec<u8>, String> {
        Err("UDP receive not implemented.".into())
    }
}