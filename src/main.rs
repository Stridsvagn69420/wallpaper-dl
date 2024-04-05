use apputils::{Colors, paintln};
use blake3::Hash;
use reqwest::Client;
use std::env;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::str::FromStr;
use std::process::ExitCode;
use url::Url;

mod downloaders;
use downloaders::{DownloaderError, Urls};

mod config;
use config::{load_db, Config, WallpaperDb, Wallpaper};

mod meta;
use meta::{Info, USER_AGENT};

mod errors;
use errors::MainErr;

/// Main Thread
/// 
/// The main function and entry of this application.
#[tokio::main(flavor = "multi_thread")]
async fn main() -> ExitCode {
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
		"current" => current(args.get(1).cloned()).await,
		_ => {
			// load config and db
			let cfg = Config::load().unwrap_or_default();
			let db = load_db().unwrap_or_default();

			let urls: Vec<Url> = args.into_iter()
				.filter_map(|x| Url::parse(&x).ok())
				.collect();
			download(cfg, db, Urls::Multi(urls)).await
		}
	}
}

/// `current` Subcommand
/// 
/// The subcommand that either displays the path to your current wallpaper or sets it.
async fn current(arg: Option<String>) -> ExitCode {
	// Load Config
	let mut wallcfg = match Config::load() {
		Ok(x) => x,
		Err(err) => {
			// User intends to read a value from a file that does not exist
			if arg.is_none() {
				return MainErr::cfg_load(err);
			}
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

	// Switch between getter and setter logic
	let Some(param) = arg else {
		// Read Hash value from config
		let Some(wcfg) = wallcfg.wallpaper else { return MainErr::cfg_wallpaper() };
		// Read entry in Database
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
			return download(wallcfg, db, Urls::Single(url)).await;
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
async fn download(config: Config, database: WallpaperDb, list: Urls) -> ExitCode {
	// Override current wallpaper in config if Single URL provided
	let override_hash = list.is_single();
	let urls = match list {
		Urls::Single(x) => vec![x],
		Urls::Multi(y) => y,
	};

	if urls.is_empty() {
		paintln!(Colors::RedBold, "No URLs provided!");
		return ExitCode::FAILURE;
	}

	let Ok(client) = Client::builder().user_agent(USER_AGENT).build() else {
		paintln!(Colors::Red, "Failed to build reqwest client!");
		return ExitCode::FAILURE;
	};

	// BEGIN Temporary proof-of-concept and blocking iterator
	for url in urls.into_iter() {
		print!("URL: ");
		paintln!(Colors::Green, "{url}");

		let walldl = match downloaders::from_url(&client, url).await {
			Ok(dl) => dl,
			Err(err) => {
				let msg = match err {
					DownloaderError::Other => "Website not supported".to_string(),
					_ => format!("An error occured during the initial request: {}", err),
				};
				paintln!(Colors::Red, "{}", msg);
				continue;
			}
		};

		print!("Title: ");
		match walldl.image_title() {
			Ok(title) => paintln!(Colors::Cyan, "{}", title),
			Err(_) => paintln!(Colors::Red, "Not found"),
		}

		print!("ID: ");
		paintln!(Colors::Cyan, "{}", walldl.image_id());

		print!("URL: ");
		match walldl.image_url() {
			Err(_) => paintln!(Colors::Red, "Could not be retrieved!"),
			Ok(url) => match url {
				Urls::Single(url) => paintln!(Colors::Red, "{url}"),
				Urls::Multi(urls) => Colors::Red.println(", ", urls)
			}
		}

		print!("Tags: ");
		match walldl.image_tags() {
			Err(_) => paintln!(Colors::Red, "Could not be retrieved!"),
			Ok(mut tags) => {
				tags.sort_unstable();
				tags.dedup();
				Colors::Blue.println(", ", tags);
			}
		}

		println!("\n{}\n", "-".repeat(50));
	}
	// END Temporary proof-of-concept and blocking iterator

	ExitCode::SUCCESS
}