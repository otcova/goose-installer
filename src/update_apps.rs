use crate::*;
use apps_info::*;
use std::process::Command;
use std::{fs, io};

/// OK -> Success, None -> failure
pub fn update_apps() -> Option<()> {
    let fetched = FetchedApps::fetch()?;
    let mut local = LocalAppInfo::new(root_folder()?).ok()?;

    // Test for apps to delete
    let local_apps = local.apps_info_ref().apps.keys();
    let apps_to_remove: Vec<String> = local_apps
        .filter(|name| !fetched.apps_info_ref().apps.contains_key(*name))
        .cloned()
        .collect();

    for name in apps_to_remove {
        let _ = remove_app(&mut local, &name);
    }

    // Test for apps to update / install
    for (name, fetched_info) in &fetched.apps_info_ref().apps {
        if let Some(local_info) = local.apps_info_ref().apps.get(name) {
            if fetched_info != local_info {
                // Update App
                if remove_app(&mut local, name).is_ok() {
                    install_app(&fetched, &mut local, name);
                }
            }
        } else {
            install_app(&fetched, &mut local, name);
        }
    }

    Some(())
}

fn remove_app(local: &mut LocalAppInfo, name: &str) -> io::Result<()> {
    let old_app_folder = local.current_app_folder(name);
    fs::remove_dir_all(old_app_folder)?;
    local.set_app_info(name, None);
    Ok(())
}

fn install_app(fetched: &FetchedApps, local: &mut LocalAppInfo, name: &str) -> Option<()> {
    fetched.download_app(local, name)?;
    let info = &fetched.apps_info_ref().apps[name];

    if let Some(exe_name) = &info.run_after_update {
        let mut exe_path = local.current_app_folder(name);
        exe_path.push(exe_name);

        let _ = Command::new("cmd")
            .args(&["/C", "start", exe_path.as_os_str().to_str()?])
            .spawn();
    }

    Some(())
}
