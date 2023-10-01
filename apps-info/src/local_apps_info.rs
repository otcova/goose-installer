use crate::*;
use anyhow::Result;
use fs2::FileExt;
use std::fs::{File, OpenOptions};
use std::io::*;
use std::path::PathBuf;

pub struct LocalAppInfo {
    apps_info: AppsInfo,
    file: File,
    root_folder: PathBuf,
    modified: bool,
}

impl LocalAppInfo {
    pub fn new(root_folder: PathBuf) -> Result<Self> {
        let path = root_folder.join("apps-info.toml");
        let file = OpenOptions::new().write(true).read(true).open(&path);

        match file {
            Ok(mut file) => {
                file.try_lock_exclusive()?;

                // Load File
                let mut raw_apps_info = String::new();
                file.read_to_string(&mut raw_apps_info)?;
                let apps_info: AppsInfo = toml::from_str(&raw_apps_info)?;

                Ok(Self {
                    file,
                    root_folder,
                    apps_info,
                    modified: false,
                })
            }
            Err(err) if err.kind() == ErrorKind::NotFound => {
                let file = File::create(path)?;
                file.try_lock_exclusive()?;
                Ok(Self {
                    file,
                    root_folder,
                    apps_info: AppsInfo::default(),
                    modified: true,
                })
            }
            Err(err) => Err(err.into()),
        }
    }

    pub fn apps_info_ref(&self) -> &AppsInfo {
        &self.apps_info
    }

    pub fn current_app_folder(&self, app_name: &str) -> PathBuf {
        self.app_folder(app_name, &self.apps_info.apps[app_name])
    }

    pub fn app_folder(&self, app_name: &str, app_info: &AppInfo) -> PathBuf {
        let folder_name = app_name.to_string() + "." + &app_info.version;
        self.root_folder.join(folder_name)
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
            self.file.write_all(updated_raw_apps_info.as_bytes())?;
        }
        Ok(())
    }
}

impl Drop for LocalAppInfo {
    fn drop(&mut self) {
        self.save_changes().unwrap();
    }
}
