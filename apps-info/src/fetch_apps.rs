use crate::*;
use anyhow::*;
use reqwest::blocking::ClientBuilder;
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
    pub fn fetch() -> Result<Self> {
        let response = reqwest::blocking::get(APPS_INFO_URL)?;
        let raw_toml = response.text()?;
        let apps_info = toml::from_str(&raw_toml)?;
        Ok(Self { apps_info })
    }

    pub fn apps_info_ref(&self) -> &AppsInfo {
        &self.apps_info
    }

    pub fn download_app(&self, local: &mut LocalAppInfo, app_name: &str) -> Result<()> {
        let info = &self.apps_info.apps[app_name];
        let app_folder = local.app_folder(app_name, info);
        let _ = fs::remove_dir_all(&app_folder);
        fs::create_dir(&app_folder)?;

        // Copy files from github to app_folder
        for (file_name, file_url) in &fetch_dir_content(app_name)? {
            download_file(file_url, app_folder.join(file_name))?;
        }

        local.set_app_info(app_name, Some(info.clone()));

        Ok(())
    }
}

#[derive(Deserialize)]
struct GitTreeEntry {
    path: String,
    #[serde(rename = "type")]
    entry_type: String,
    url: String,
}

#[derive(Deserialize)]
struct GitBranch {
    tree: Vec<GitTreeEntry>,
}

fn fetch_dir_content(app_name: &str) -> Result<Vec<(String, String)>> {
    let client = ClientBuilder::new().user_agent("otcova").build()?;
    let repo_tree: GitBranch = serde_json::from_str(&client.get(REPO_TREE_URL).send()?.text()?)?;
    let app_path = app_name.to_string() + "/";

    let files = repo_tree
        .tree
        .into_iter()
        .filter_map(|entry| {
            if entry.path.starts_with(&app_path) && entry.entry_type == "blob" {
                Some((entry.path[app_path.len()..].to_string(), entry.url))
            } else {
                None
            }
        })
        .collect();

    Ok(files)
}

fn download_file<P: AsRef<Path>>(file_url: &str, dst: P) -> Result<()> {
    let file_blob = reqwest::blocking::get(file_url)?.bytes()?;
    if let Some(parent) = dst.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(fs::write(dst, file_blob.as_ref())?)
}
