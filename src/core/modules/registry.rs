use crate::core::modules::Module;
use crate::core::modules::linux::whoami::WhoamiLinux;
use crate::core::modules::linux::uname::UnameLinux;
use crate::core::modules::linux::download::DownloadLinux;
use crate::core::modules::windows::whoami::WhoamiWindows;
use crate::core::modules::windows::systeminfo::SysteminfoWindows;
use crate::core::modules::windows::download::DownloadWindows;

pub fn get_modules() -> Vec<Box<dyn Module>> {
    vec![
        Box::new(WhoamiLinux),
        Box::new(UnameLinux),
        Box::new(DownloadLinux),
        Box::new(WhoamiWindows),
        Box::new(SysteminfoWindows),
        Box::new(DownloadWindows),
    ]
}