#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod setup_installer;
mod update_apps;

use std::env::current_exe;
use std::path::*;

fn main() {
    setup_installer::startup_schedule();

    loop {
        update_apps::update_apps();
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

fn root_folder() -> Option<PathBuf> {
    let mut exe_path = current_exe().ok()?;
    exe_path.pop(); // Pop installer.exe
    exe_path.pop(); // Pop instller_folder/installer.exe
    Some(exe_path)
}
