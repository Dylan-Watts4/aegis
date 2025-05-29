use crate::core::modules::Module;
use crate::core::modules::linux::whoami::WhoamiLinux;
use crate::core::modules::linux::uname::UnameLinux;
use crate::core::modules::linux::download::DownloadLinux;
use crate::core::modules::linux::upload::UploadLinux;
use crate::core::modules::linux::netstat::NetstatLinux;
use crate::core::modules::linux::ps::PsLinux;
use crate::core::modules::linux::interactive::InteractiveShellLinux;
use crate::core::modules::linux::sudo_check::SudoCheckLinux;
use crate::core::modules::linux::linpeas::LinpeasLinux;
use crate::core::modules::linux::shadow_dump::ShadowDumpLinux;

use crate::core::modules::windows::whoami::WhoamiWindows;
use crate::core::modules::windows::systeminfo::SysteminfoWindows;
use crate::core::modules::windows::download::DownloadWindows;
use crate::core::modules::windows::upload::UploadWindows;
use crate::core::modules::windows::netstat::NetstatWindows;
use crate::core::modules::windows::tasklist::TasklistWindows;
use crate::core::modules::windows::winpeas::WinpeasWindows;
use crate::core::modules::windows::sam_dump::SamDumpWindows;
use crate::core::modules::windows::lsass_dump::LsassDumpWindows;

pub fn get_modules() -> Vec<Box<dyn Module>> {
    vec![
        Box::new(WhoamiLinux),
        Box::new(UnameLinux),
        Box::new(DownloadLinux),
        Box::new(UploadLinux),
        Box::new(NetstatLinux),
        Box::new(PsLinux),
        Box::new(InteractiveShellLinux),
        Box::new(SudoCheckLinux),
        Box::new(LinpeasLinux),
        Box::new(ShadowDumpLinux),
        Box::new(WhoamiWindows),
        Box::new(SysteminfoWindows),
        Box::new(DownloadWindows),
        Box::new(UploadWindows),
        Box::new(NetstatWindows),
        Box::new(TasklistWindows),
        Box::new(WinpeasWindows),
        Box::new(SamDumpWindows),
        Box::new(LsassDumpWindows),
    ]
}