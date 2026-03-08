use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MboxConfig {
    pub settings: Settings,
    pub accounts: std::collections::BTreeMap<String, AccountConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub output_dir: String,
    pub qlever_dir: Option<String>,
    pub compress: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountConfig {
    pub email: String,
    pub graph: String,
    #[serde(default)]
    pub folders: Vec<FolderConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderConfig {
    pub name: String,
    pub path: String,
    pub include: bool,
}
