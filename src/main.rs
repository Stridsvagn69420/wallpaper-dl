use std::env;
use std::process::ExitCode;
use apputils::Colors;
use apputils::paintln;
use reqwest::Client;
use url::Url;

mod downloaders;
use downloaders::{Downloader, WallpaperAbyss};

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), '/', env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() -> ExitCode {
	let args: Vec<Url> = env::args().filter_map(|x| Url::parse(&x).ok()).collect();
	if args.len() < 2 {
		paintln!(Colors::Red, "No URLs provided!");
		return ExitCode::FAILURE;
	}

	let Ok(client) = Client::builder().user_agent(USER_AGENT).build() else {
		paintln!(Colors::Red, "Failed to build reqwest client!");
		return ExitCode::FAILURE;
	};

	// Temporary test for Alphacoders
	for url in args.into_iter().filter(|x| x.host_str().is_some_and(|y| y == "wall.alphacoders.com")) {
		paintln!(Colors::Green, "URL: {}", url);

		paintln!(Colors::Yellow, "Downloading...");
		let Ok(abyss) = WallpaperAbyss::new(&client, url).await else {
			paintln!(Colors::Red, "Failed to download image!");
			return ExitCode::FAILURE;
		};

		print!("Title: ");
		match abyss.image_title() {
			Ok(title ) => paintln!(Colors::Cyan, "{}", title),
			Err(_) => paintln!(Colors::Red, "Not found")
		}

		print!("ID: ");
		paintln!(Colors::Cyan, "{}", abyss.image_id());

		print!("URL: ");
		match abyss.image_url() {
			Ok(url ) => paintln!(Colors::Cyan, "{}", url),
			Err(_) => paintln!(Colors::Red, "Could not be retrieved!")
		}
	}

	ExitCode::SUCCESS
}