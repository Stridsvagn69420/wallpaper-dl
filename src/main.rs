use apputils::{paint, paintln, Colors};
use blake3::{hash, Hash};
use reqwest::blocking::Client;
use std::{env, path::Path};
use std::io::ErrorKind;
use std::path::PathBuf;
use std::str::FromStr;
use std::process::ExitCode;
use url::Url;

mod downloaders;
use downloaders::{Urls, WallpaperMeta};

mod config;
use config::{load_db, save_db, Config, Wallpaper, WallpaperDb, WallpaperEntry};

mod meta;
use meta::{Info, USER_AGENT};

mod wrappers;
use wrappers::{MainErr, DownErr, Filedown};

/// Main Thread
/// 
/// The main function and entry of this application.
fn main() -> ExitCode {
	// Get Command-Line Args excluding the binary path
	let args: Vec<String> = env::args_os()
		.skip(1)
		.filter_map(|x| x.into_string().ok())
		.collect();

	// Parse first argument
	let Some(cmd) = args.first() else {
		return Info::help(true);
	};
	match cmd.as_str() {
		"-h" | "--help" => Info::help(false),
		"-V" | "--version" => Info::version(),
		"current" => current(args.get(1).cloned()),
		_ => {
			// load config and db
			let cfg = Config::load().unwrap_or_default();
			let db = load_db().unwrap_or_default();

			let urls: Vec<Url> = args.into_iter()
				.filter_map(|x| Url::parse(&x).ok())
				.collect();
			download(cfg, db, Urls::Multi(urls))
		}
	}
}

/// `current` Subcommand
/// 
/// The subcommand that either displays the path to your current wallpaper or sets it.
fn current(arg: Option<String>) -> ExitCode {
	// Load Config
	let mut wallcfg = match Config::load() {
		Ok(x) => x,
		Err(err) => {
			// User intends to read a value from a file that does not exist
			if arg.is_none() { return MainErr::cfg_load(err); }
			Config::default()
		}
	};

	// Load database (absolutely needed for this)
	let db = match load_db() {
		Ok(x) => x,
		Err(err) => return match err.kind() {
			ErrorKind::NotFound => MainErr::db_load_notfound(),
			_ => MainErr::db_load()
		}
	};

	// Switch between Getter and Setter logic
	let Some(param) = arg else {
		// Read Hash value from Config, then its entry in Database
		let Some(wcfg) = wallcfg.wallpaper else { return MainErr::cfg_wallpaper() };
		let Some(wdb) = db.get(&wcfg.current) else { return MainErr::db_not_found(Colors::Yellow, "Wallpaper") };

		// Just print
		println!("{}", wdb.file.as_path().to_string_lossy());
		return ExitCode::SUCCESS;
	};

	// Parse the parameter
	let newhash = if let Ok(url) = Url::parse(&param) {
		// Find Hash Key by Source property, download Wallpaper if not found
		let Some(entry) = db.iter().find(|x| x.1.source == param) else {
			MainErr::db_param_not_found(Colors::Magenta, Colors::MagentaBold, "URL", &url);
			return download(wallcfg, db, Urls::Single(url));
		};
		entry.0.to_owned()
	} else if let Ok(hash) = Hash::from_str(&param) {
		// Check if Hash exists and update config
		if db.get(&hash).is_none() {
			return MainErr::db_param_not_found(Colors::Red, Colors::RedBold, "Hash", hash);
		}
		hash
	} else {
		// Find Hash Key by File property
		let path = PathBuf::from(&param);
		let Some(entry) = db.iter().find(|x| x.1.file == path) else {
			return MainErr::db_param_not_found(Colors::Red, Colors::RedBold, "File", &param);
		};
		entry.0.to_owned()
	};

	// Attempt to save config
	wallcfg.wallpaper = Some(Wallpaper { current: newhash });
	match wallcfg.save() {
		Ok(_) => ExitCode::SUCCESS,
		Err(err) => MainErr::cfg_save(Colors::RedBold, Colors::Red, err)
	}
}

/// Downloader loop
/// 
/// The main function that downloads the provided images
/// and prints status information to the console.
/// 
/// Currently it does not use any asynchronous functionality.
fn download(mut config: Config, mut database: WallpaperDb, list: Urls) -> ExitCode {
	// Keep original version of config for write failure importance
	let update_hash = list.is_single();

	// Parse input strings to URLs
	let mut allurls: Vec<Url> = list.into();
	if allurls.is_empty() {
		return DownErr::valid_urls();
	}
	allurls.sort_unstable();
	allurls.dedup();
	
	// Filter out already existing Wallpapers
	let already_exists: Vec<Url> = database.values()
		.filter_map(|x| Url::from_str(&x.source).ok())
		.collect();
	let urls: Vec<Url> = allurls.into_iter()
		.filter(|x| already_exists.binary_search(x).is_ok())
		.collect();
	if urls.is_empty() {
		return DownErr::new_urls();
	}

	// Create HTTP Client
	let Ok(client) = Client::builder().user_agent(USER_AGENT).build() else {
		return DownErr::tls_resolve();
	};

	// 1. Fetch metadata of all provided wallpapers
	let wallmetadata: Vec<(Url, WallpaperMeta, String)> = urls.into_iter()
	.filter_map(|link| {
		let host = link.host_str().unwrap_or("Website").to_owned();
		let meta = match downloaders::from_url(&client, link.clone()).and_then(TryInto::<WallpaperMeta>::try_into) {
			Ok(x) => x,
			Err(err) => {
				return DownErr::init_req(err, &host);
			}
		};
		Some((link, meta, host))
	})
	.collect();

	if wallmetadata.is_empty() {
		return ExitCode::FAILURE;
	}

	// 2. Download wallpapers
	for (source, wallmeta, host) in wallmetadata {
		// Counter for indexed filenames
		let single = wallmeta.images.is_single();
		let mut count: u8 = 0;

		for wall in Vec::from(&wallmeta.images) {
			count += 1;
			paint!(Colors::Cyan, "  Downloading ");
			paint!(Colors::CyanBold, "{}@{host}", wallmeta.id);

			let Ok(data) = Filedown::download_file(&client, &wallmeta, wall) else {
				paintln!(Colors::RedBold, " FAILED");
				continue;
			};
			paint!(Colors::MagentaBold, " ↓ ");

			// Format Filename and Folder Path
			let file = if single {
				Filedown::format_file(&wallmeta, data.1, data.2, None)
			} else {
				Filedown::format_file(&wallmeta, data.1, data.2, Some(count))
			};
			let dir = Filedown::resolve_path(&config, &host, &wallmeta.tags);

			// Save file data
			if Filedown::save_file(dir, &file, &data.0).is_err() {
				paintln!(Colors::Red, " Failed to write to disk!");
				continue;
			}

			// Update database
			let filehash = hash(&data.0);
			let entry = WallpaperEntry {
				source: source.to_string(),
				file: Path::new(&file).to_path_buf()
			};
			database.insert(filehash, entry);
			paintln!(Colors::BlueBold, "Saved!");

			// Update config
			if update_hash {
				config.wallpaper = Some(Wallpaper { current: filehash });
			}
		}
	}

	// 3. Result
	let database_store_fail = save_db(&database).is_err();
	let config_store_failed = config.save().is_err() && update_hash;
	DownErr::finish(config_store_failed, database_store_fail)
}