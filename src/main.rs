use apputils::{paint, paintln, Colors};
use reqwest::{blocking::Client, header::CONTENT_TYPE};
use std::error::Error;
use std::io;
use std::fs::File;
use std::env;
use std::process::ExitCode;
use url::Url;

mod downloaders;
use downloaders::{WallpaperMeta, DownloaderError};

mod meta;
use meta::{Info, USER_AGENT};

/// Main Thread
/// 
/// The main function and entry of this application.
fn main() -> ExitCode {
	// Get Command-Line Args excluding the binary path
	let args: Vec<String> = env::args_os()
		.skip(1)
		.filter_map(|x| x.into_string().ok())
		.collect();

	// Handle flags
	let Some(cmd) = args.first() else {
		return Info::help(true);
	};
	match cmd.as_str() {
		"-h" | "--help" => Info::help(false),
		"-V" | "--version" => Info::version(),
		_ => {
			let urls: Vec<Url> = args.into_iter()
				.filter_map(|x| Url::parse(&x).ok())
				.collect();
			action(urls)
		}
	}
}

fn action(urls: Vec<Url>) -> ExitCode {
	if urls.is_empty() {
		paintln!(Colors::YellowBold, "No valid URLs were provided!");
		return ExitCode::FAILURE;
	}

	// Create HTTP Client
	let Ok(client) = Client::builder().user_agent(USER_AGENT).build() else {
		paintln!(Colors::RedBold, "Failed to initialize HTTP-Client!");
		return ExitCode::FAILURE;
	};

	// Fecth Wallpaper Metadata
	let mut wallmetadata: Vec<(Url, WallpaperMeta, String)> = Vec::new();
	for link in urls {
		let host = link.host_str().unwrap_or("Website").to_owned();
		
		paint!(Colors::GreenBold, "  Fetching ");
		print!("{link}");

		let meta = match downloaders::from_url(&client, link.clone()).and_then(TryInto::<WallpaperMeta>::try_into) {
			Ok(x) => x,
			Err(err) => {
				match err {
					DownloaderError::Other => paintln!(Colors::YellowBold, " Not Supported!"),
					_ => paintln!(Colors::RedBold, " Failed: {}{err}", Colors::Red)
				};
				continue;
			}
		};
		println!();
		wallmetadata.push((link, meta, host));
	}

	// Check if everything failed
	if wallmetadata.is_empty() {
		return ExitCode::FAILURE;
	}

	// Download wallpapers
	let total = wallmetadata.len();
	let mut errors = 0;
	for (source, wallmeta, host) in wallmetadata {
		// Counter for indexed filenames
		let single = wallmeta.images.len() < 2;
		let mut count: u8 = 1;
		let mut failed = false;

		for wall in wallmeta.images {
			paint!(Colors::CyanBold, "  Downloading ");
			print!("{source}");

			// Download file
			match download(&client, wall, &wallmeta.id, &host, (single, count)) {
				Ok(fname) => paintln!(Colors::Blue, " Saved as {}{fname}", Colors::BlueBold),
				Err(_) => {
					paintln!(Colors::RedBold, " FAILED");
					failed = true;
				}
			}
			count += 1;
		}
		// Error statistic
		if failed {
			errors += 1;
		}
	}

	// Error code
	if errors > total / 2 {
		ExitCode::FAILURE
	} else {
		ExitCode::SUCCESS
	}
}

fn download(client: &Client, url: Url, id: &str, host: &str, ctr: (bool, u8)) -> Result<String, Box<dyn Error>> {
	let resp = client.get(url).send()?;

	// Get file extension
	let ext = match resp.headers().get(CONTENT_TYPE).and_then(|x| x.to_str().ok()) {
		None => "bin",
		Some(ctype) => match ctype {
			"image/bmp" => "bmp",
			"image/apng" => "apng",
			"image/png" => "png",
			"image/jpeg" => "jpg",
			"image/gif" => "gif",
			"image/avif" => "avif",
			"image/webp" => "webp",
			_ => "bin"
		}
	};

	// Build filename
	let fname = if ctr.0 {
		format!("{id}_{host}.{ext}")
	} else {
		format!("{id}_{}_{host}.{ext}", ctr.1)
	};

	// Download file data
	let mut data = io::Cursor::new(resp.bytes()?);
	let mut file = File::create(&fname)?;
	io::copy(&mut data, &mut file)?;

	Ok(fname)
}