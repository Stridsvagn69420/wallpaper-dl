use crate::meta::{APP_NAME, CONFIG_FILE, WALLPAPERS_FILE};
use apputils::config::{Appdata, Cfg};
use apputils::dirs::home_dir;
use blake3::Hash;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use toml;

/// Map to io-Error
///
/// Could've used a macro I guess
fn to_io_err(err: impl Error + Send + Sync + 'static) -> io::Error {
	io::Error::new(io::ErrorKind::Other, err)
}

/// Wallpaper Database alias
///
/// The type for `wallpapers.toml`'s structure.
pub type WallpaperDb = HashMap<Hash, WallpaperEntry>;

/// Wallpaper Database entry
///
/// The entry in `wallpapers.toml`'s map.
#[derive(Deserialize, Serialize)]
pub struct WallpaperEntry {
	source: String,
	file: PathBuf,
}

/// Load wallpaper database table
///
/// Loads the `wallpapers.toml` file.
pub fn load_db() -> io::Result<WallpaperDb> {
	let dbstr = Appdata::read_str(APP_NAME, WALLPAPERS_FILE)?;
	toml::from_str(&dbstr).map_err(to_io_err)
}

/// Save wallpaper database table
///
/// Saves the modified database to `wallpapers.toml`.
pub fn save_db(db: &WallpaperDb) -> io::Result<()> {
	let dbtxt = toml::to_string_pretty(db).map_err(to_io_err)?;
	Appdata::save(APP_NAME, WALLPAPERS_FILE, dbtxt)
}

/// Config super struct
///
/// The struct that combines all the sub config structs.
#[derive(Deserialize, Serialize, Default)]
pub struct Config {
	/// Filesystem config
	pub filesystem: Filesystem,

	/// Genre Map
	///
	/// The genre table for sorting by genre.
	/// Each entry's key is the name of the folder and its value is an array of image keywords/tags to match for that folder.
	pub genres: Option<HashMap<String, Vec<String>>>,

	/// Wallpaper config
	///
	/// Your current wallpaper. Used to make `wallpaper-dl`
	/// interactive with other applications via Bash scripting.
	pub wallpaper: Option<Wallpaper>,
}

impl Config {
	/// Load config file
	///
	/// Loads the config using the [config backend](apputils::config::Cfg) by [apputils].
	pub fn load() -> io::Result<Self> {
		let tomltext = Cfg::read(APP_NAME, CONFIG_FILE)?;
		toml::from_str(&tomltext).map_err(to_io_err)
	}

	/// Save config file
	///
	/// Saves the current config file as TOML.
	pub fn save(&self) -> io::Result<()> {
		let cfgtxt = toml::to_string_pretty(self).map_err(to_io_err)?;
		Cfg::save(APP_NAME, CONFIG_FILE, cfgtxt)
	}
}

/// Wallpaper config
///
/// The Wallpaper sub-struct.
#[derive(Deserialize, Serialize)]
pub struct Wallpaper {
	/// Current Wallpaper Hash
	///
	/// The [Hash](struct@Hash) of the currently selected wallpaper inside `wallpapers.toml`.
	pub current: Hash,
}

/// Filesystem settings
///
/// Sub-struct for configuring filesystem related settings.
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Filesystem {
	pub path: PathBuf,
	pub sort: Sort,
}

impl Default for Filesystem {
	fn default() -> Self {
		Self {
			// 99% of the time $HOME is set, so unless you fucked up basic Linux commands, this won't panic.
			path: home_dir().unwrap().join("Pictures"),
			sort: Sort::Hostname,
		}
	}
}

/// Sorting Methods
#[derive(Deserialize, Serialize)]
#[serde(rename_all_fields = "lowercase")]
pub enum Sort {
	Hostname,
	Genres,
	None,
}