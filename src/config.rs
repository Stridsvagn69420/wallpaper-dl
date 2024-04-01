use crate::meta::{APP_NAME, CONFIG_FILE};
use std::io;
use std::collections::HashMap;
use std::path::PathBuf;
use std::default::Default;
use apputils::config::local_file;
use apputils::dirs::home_dir;
use serde::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct Config {
	/// Filesystem config
    pub filesystem: Filesystem,

	/// Genre Map
	/// 
	/// The genre table for sorting by genre.
	/// Each entry's key is the name of the folder and its value is an array of image keywords/tags to match for that folder.
	pub genres: Option<HashMap<String, Vec<String>>>
}

impl Config {
    /// Load standard config
    ///
    /// Loads the config using the [config backend](apputils::config) by [apputils].
    pub fn load() -> io::Result<Self> {
        let tomltext = local_file(APP_NAME, CONFIG_FILE)?;
        toml::from_str(&tomltext).map_err(|x| io::Error::new(io::ErrorKind::Other, x))
    }
}

impl Default for Config {
	fn default() -> Self {
		Self {
			filesystem: Filesystem::default(),
			genres: None
		}
	}
}

/// Filesystem settings
/// 
/// Sub-struct for configuring filesystem related settings.
#[derive(Deserialize)]
pub struct Filesystem {
    pub path: PathBuf,
    pub sort: Sort
}

impl Default for Filesystem {
	fn default() -> Self {
		Self {
			// 99% of the time $HOME is set, so unless you fucked up basic Linux commands, this won't panic.
			path: home_dir().unwrap().join("Pictures"),
			sort: Sort::Hostname
		}
	}
}

/// Sorting Methods
#[derive(Deserialize)]
#[serde(rename_all_fields = "lowercase")]
pub enum Sort {
    Hostname,
    Genre,
    None
}
