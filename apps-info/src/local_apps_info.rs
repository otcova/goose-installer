use crate::*;
use anyhow::Result;
use std::fs;
use std::io::*;
use std::path::{Path, PathBuf};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;

pub struct LocalAppInfo {
    apps_info: AppsInfo,
    root_folder: PathBuf,
    modified: bool,
}

fn apps_info_path(root_folder: &Path) -> PathBuf {
    root_folder.join("apps-info.toml")
}

impl LocalAppInfo {
    pub fn new(root_folder: PathBuf) -> Result<Self> {
        let apps_info_path = apps_info_path(&root_folder);

        match fs::read_to_string(&apps_info_path) {
            Ok(raw_apps_info) => {
                // Load File
                if raw_apps_info.is_empty() {
                    Ok(Self {
                        root_folder,
                        apps_info: AppsInfo::default(),
                        modified: true,
                    })
                } else {
                    let apps_info: AppsInfo = toml::from_str(&raw_apps_info)?;

                    Ok(Self {
                        root_folder,
                        apps_info,
                        modified: false,
                    })
                }
            }
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(Self {
                root_folder,
                apps_info: AppsInfo::default(),
                modified: true,
            }),
            Err(err) => Err(err.into()),
        }
    }

    pub fn apps_info_ref(&self) -> &AppsInfo {
        &self.apps_info
    }

    pub fn root_folder(&self) -> &Path {
        &self.root_folder
    }

    pub fn app_folder(&self, app_name: &str) -> PathBuf {
        app_folder(app_name, &self.apps_info.apps[app_name], &self.root_folder)
    }

    pub fn set_app_info(&mut self, app_name: &str, app_info: Option<AppInfo>) {
        // Update App info
        self.modified = true;
        if let Some(info) = app_info {
            self.apps_info.apps.insert(app_name.to_string(), info);
        } else {
            self.apps_info.apps.remove(app_name);
        }
    }

    /// It will be called on Drop
    pub fn save_changes(&mut self) -> Result<()> {
        if self.modified {
            self.modified = false;
            let updated_raw_apps_info = toml::to_string_pretty(&self.apps_info)?;
            let path = apps_info_path(&self.root_folder);
            fs::write(path, updated_raw_apps_info)?;
        }
        Ok(())
    }
}

pub fn app_folder(app_name: &str, app_info: &AppInfo, root_folder: &Path) -> PathBuf {
    let folder_name = app_name.to_string() + "." + &app_info.version;
    root_folder.join(folder_name)
}

impl Drop for LocalAppInfo {
    fn drop(&mut self) {
        self.save_changes().unwrap();
    }
}

pub fn start_detached_admin_process(exe_path: &Path) -> Result<()> {
    unsafe {
        use std::os::windows::ffi::OsStrExt;
        use windows::core::*;

        let mut exe_path: Vec<u16> = exe_path.as_os_str().encode_wide().collect();
        exe_path.push(0);
        let exe_path = PCWSTR(exe_path.as_mut_ptr());

        ShellExecuteW(None, w!("runas"), exe_path, None, None, SW_HIDE);
    }
    Ok(())
}
