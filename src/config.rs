use crate::meta::{APP_NAME, CONFIG_FILE, WALLPAPERS_FILE};
use apputils::config::{local_dir, local_file};
use apputils::dirs::{data_home, home_dir};
use blake3::Hash;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use toml;

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

fn to_io_err(err: impl Error + Send + Sync + 'static) -> io::Error {
	io::Error::new(io::ErrorKind::Other, err)
}

impl Config {
	fn store(cfg: &impl Serialize, path: impl AsRef<Path>) -> io::Result<()> {
		let dbstr = toml::to_string_pretty(cfg).map_err(to_io_err)?;
		fs::write(path, dbstr)
	}

	/// Load config file
	///
	/// Loads the config using the [config backend](apputils::config) by [apputils].
	pub fn load() -> io::Result<Self> {
		let tomltext = local_file(APP_NAME, CONFIG_FILE)?;
		toml::from_str(&tomltext).map_err(to_io_err)
	}

	/// Save config file
	///
	/// Saves the current config file as TOML.
	pub fn save(&self) -> io::Result<()> {
		let path = local_dir(APP_NAME).unwrap().join(CONFIG_FILE);
		Self::store(self, path)
	}

	/// Database Path
	///
	/// Returns a [PathBuf] with the path to `wallpapers.toml`.
	fn db_path() -> PathBuf {
		// Can be unwrapped, panics should pratically not happen.
		data_home().unwrap().join(APP_NAME).join(WALLPAPERS_FILE)
	}

	/// Load wallpaper database table
	///
	/// Loads the `wallpapers.toml` file.
	pub fn load_db() -> io::Result<WallpaperDb> {
		let dbstr = fs::read_to_string(Self::db_path())?;
		toml::from_str(&dbstr).map_err(to_io_err)
	}

	/// Save wallpaper database table
	///
	/// Saves the modified database to `wallpapers.toml`.
	pub fn save_db(db: &WallpaperDb) -> io::Result<()> {
		Self::store(db, Self::db_path())
	}
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
	Genre,
	None,
}