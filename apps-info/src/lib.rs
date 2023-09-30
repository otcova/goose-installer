mod fetch_apps;
mod local_apps_info;

pub use fetch_apps::*;
pub use local_apps_info::*;

use serde::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct AppsInfo {
    pub apps: HashMap<String, AppInfo>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct AppInfo {
    pub version: String,
    pub run_after_update: Option<String>,
}
