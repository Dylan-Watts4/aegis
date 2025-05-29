pub mod tcp;
pub mod manager;
pub mod udp;

pub trait Comms: Send + 'static {
    fn start(&self);
    fn send(&self, session_id: usize, data: &[u8]) -> Result<(), String>;
    fn receive(&self, session_id: usize) -> Result<Vec<u8>, String>;
}