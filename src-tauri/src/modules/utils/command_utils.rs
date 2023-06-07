
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Command;
#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub trait NoWindow {
    fn no_window(&mut self) -> &mut Self;
}


impl NoWindow for Command {
    fn no_window(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        self.creation_flags(CREATE_NO_WINDOW);
        self
    }
}