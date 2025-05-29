use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::net::SocketAddr;
use std::time::{SystemTime};

pub type SessionId = usize;

#[derive(Debug, Clone)]
pub enum Protocol {
    TCP,
    UDP,
    HTTP,
}

#[derive(Debug)]
pub struct Session {
    pub id: SessionId,
    pub protocol: Protocol,
    pub stream: Arc<Mutex<TcpStream>>, // TODO: Abstract for other protocols
    pub remote_addr: Option<SocketAddr>,
    pub start_time: SystemTime,
    pub last_active: Mutex<SystemTime>,
}

impl Clone for Session {
    fn clone(&self) -> Self {
        Session {
            id: self.id,
            protocol: self.protocol.clone(),
            stream: Arc::clone(&self.stream),
            remote_addr: self.remote_addr,
            start_time: self.start_time,
            last_active: Mutex::new(*self.last_active.lock().unwrap()),
        }
    }
}

pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<SessionId, Arc<Session>>>>,
    next_id: Arc<Mutex<SessionId>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(0)),
        }
    }

    pub fn add_tcp_session(&self, stream: TcpStream) -> SessionId {
        let mut id_lock = self.next_id.lock().unwrap();
        let session_id = *id_lock;
        *id_lock += 1;
        stream.set_nonblocking(true).ok();

        let remote_addr = stream.peer_addr().ok();
        let now = SystemTime::now();

        let session = Arc::new(Session {
            id: session_id,
            protocol: Protocol::TCP,
            stream: Arc::new(Mutex::new(stream)),
            remote_addr,
            start_time: now,
            last_active: Mutex::new(now),
        });

        self.sessions.lock().unwrap().insert(session_id, session);
        session_id
    }

    pub fn get(&self, id: SessionId) -> Option<Arc<Session>> {
        self.sessions.lock().unwrap().get(&id).cloned()
    }

    pub fn remove(&self, id: SessionId) -> Result<(), String> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.remove(&id) {
            match session.protocol {
                Protocol::TCP => {
                    let stream = session.stream.lock().unwrap();
                    stream.shutdown(std::net::Shutdown::Both)
                        .map_err(|e| format!("Failed to shut down TCP stream: {}", e))?;
                }
                _ => {
                    // Placeholder for other protocols
                    return Err("Unsupported protocol for removal".into());
                }
            }
            println!("[*] Session {} closed", id);
            Ok(())
        } else {
            Err(format!("No session found with ID {}", id))
        }
    }

    pub fn list_sessions(&self) -> Vec<Arc<Session>> {
        self.sessions.lock().unwrap().values().cloned().collect()
    }
}