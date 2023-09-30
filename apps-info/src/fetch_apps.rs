use crate::*;
use std::fs;
use std::path::Path;

const APPS_INFO_URL: &'static str =
    "https://raw.githubusercontent.com/otcova/goose-installer/apps/apps-info.toml";
const REPO_TREE_URL: &'static str =
    "https://api.github.com/repos/otcova/goose-installer/git/trees/apps?recursive=1";

pub struct FetchedApps {
    apps_info: AppsInfo,
}

impl FetchedApps {
    pub fn fetch() -> Option<Self> {
        let response = reqwest::blocking::get(APPS_INFO_URL).ok()?;
        let raw_toml = response.text().ok()?;
        let apps_info = toml::from_str(&raw_toml).ok()?;
        Some(Self { apps_info })
    }

    pub fn apps_info_ref(&self) -> &AppsInfo {
        &self.apps_info
    }

    pub fn download_app(&self, local: &mut LocalAppInfo, app_name: &str) -> Option<()> {
        let info = &self.apps_info.apps[app_name];
        let app_folder = local.app_folder(app_name, info);
        let _ = fs::remove_dir_all(&app_folder);
        fs::create_dir(&app_folder).ok()?;

        // Copy files from github to app_folder
        for (file_name, file_url) in &fetch_dir_content(app_name)? {
            download_file(file_url, app_folder.join(file_name));
        }

        local.set_app_info(app_name, Some(info.clone()));

        Some(())
    }
}

fn fetch_dir_content(app_name: &str) -> Option<Vec<(String, String)>> {
    let repo_tree = reqwest::blocking::get(REPO_TREE_URL).ok()?.text().ok()?;
    let app_path = app_name.to_string() + "/";
    let mut files = Vec::new();

    for (pat_start, pat) in repo_tree.match_indices(r#""path": ""#) {
        let path_slice = &repo_tree[pat_start + pat.len()..];
        let path_slice = &path_slice[..path_slice.find("}")?];

        // Is not an entry from the app_name folder
        if !path_slice.starts_with(&app_path) {
            continue;
        }

        // Is not a file
        let entry_type = &path_slice[path_slice.find(r#""type": ""#)?..];
        if !entry_type.starts_with("blob") {
            continue;
        }

        let file_name = &path_slice[app_path.len()..path_slice.find('"')?];

        let url = &path_slice[path_slice.find(r#""url": ""#)?..];
        let url = &url[..url.find('"')?];

        files.push((file_name.to_string(), url.to_string()));
    }

    Some(files)
}

fn download_file<P: AsRef<Path>>(file_url: &str, dst: P) -> Option<()> {
    let file_blob = reqwest::blocking::get(file_url).ok()?;
    fs::write(dst, file_blob.bytes().ok()?.as_ref()).ok()
}
