use super::errors::*;
use serde::ser::SerializeStruct;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const EAM_DATA_DIR: &str = "eso-addons";
pub const EAM_CONF: &str = "config.json";
pub const EAM_DB: &str = "addons.db";

#[derive(Deserialize, Debug, Clone)]
pub struct AddonEntry {
    pub name: String,
    pub url: Option<String>,
    #[serde(default = "default_dependency")]
    pub dependency: bool,
}

fn default_dependency() -> bool {
    false
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "addonDir")]
    pub addon_dir: PathBuf,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addons: Vec<AddonEntry>,
    #[serde(rename = "FileList", default = "default_str")]
    pub file_list: String,
    #[serde(rename = "FileDetails", default = "default_str")]
    pub file_details: String,
    #[serde(rename = "ListFiles", default = "default_str")]
    pub list_files: String,
}

fn default_str() -> String {
    "".to_string()
}

impl serde::Serialize for AddonEntry {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("AddonEntry", 0)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("url", &self.url)?;
        state.serialize_field("dependency", &self.dependency)?;

        state.end()
    }
}

pub fn parse_config(path: &Path) -> Result<Config> {
    if !path.exists() {
        create_initial_config(path)?;
    }

    let config_data = fs::read_to_string(path).map_err(|_| Error::CannotLoadConfig)?;
    let config: Config = serde_json::from_str(&config_data).map_err(|_| Error::CannotLoadConfig)?;
    Ok(config)
}

pub fn save_config(path: &Path, cfg: &Config) -> Result<()> {
    let config_str =
        serde_json::to_string_pretty(cfg).map_err(|err| Error::Other(Box::new(err)))?;
    fs::write(path, config_str)?;
    Ok(())
}

fn create_initial_config(path: &Path) -> Result<()> {
    let config = get_initial_config();
    save_config(path, &config)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn get_initial_config() -> Config {
    let home_dir = dirs::home_dir().unwrap();
    let addon_dir = home_dir.join("Documents/Elder Scrolls Online/live/AddOns");

    Config {
        addon_dir: addon_dir,
        addons: vec![],
    }
}

#[cfg(target_os = "linux")]
fn get_initial_config() -> Config {
    // steam deck defaults
    let hostname = hostname::get().unwrap().into_string().unwrap();
    let addon_dir = match hostname.as_str() {
        "steamdeck" => PathBuf::from("/home/deck/.local/share/Steam/steamapps/compatdata/306130/pfx/drive_c/users/steamuser/My Documents/Elder Scrolls Online/live/AddOns"),
        _ => dirs::home_dir().unwrap().join("drive_c/users/user/My Documents/Elder Scrolls Online/live/AddOns")
    };

    Config {
        addon_dir: addon_dir,
        addons: vec![],
        file_details: "".to_string(),
        file_list: "".to_string(),
        list_files: "".to_string(),
    }
}

#[cfg(target_os = "macos")]
fn get_initial_config() -> Config {
    Config {
        addon_dir: PathBuf::new(),
        addons: vec![],
    }
}
