pub mod linux;
pub mod windows;
pub mod registry;

pub trait Module {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn usage(&self) -> &'static str;
    fn run(&self, session_id: usize, session_manager: &crate::core::session::SessionManager, args: Vec<String>);
}