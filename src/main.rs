use std::env;
use std::process::ExitCode;
use apputils::Colors;
use apputils::paintln;
use reqwest::{Client, Url};

mod extractors;
mod alphacoders;
use alphacoders::WallpaperAbyss;

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

		paintln!(Colors:: Yellow, "Downloading...");

		let Ok(resp) = client.get(url.clone()).send().await else {
			paintln!(Colors::Red, "Failed to download HTML source!");
			return ExitCode::FAILURE;
		};

		let Ok(html) = resp.text().await else {
			paintln!(Colors::Red, "Download content was not HTML text!");
			return ExitCode::FAILURE;
		};

		let Some(abyss) = WallpaperAbyss::new(&html, &url) else {
			paintln!(Colors::Red, "WallpaperAbyss failed to initialize!");
			return ExitCode::FAILURE;
		};

		print!("Title: ");
		paintln!(Colors::Cyan, "{}", abyss.image_title().unwrap());

		print!("ID: ");
		paintln!(Colors::Cyan, "{}", abyss.image_id());

		print!("URL: ");
		paintln!(Colors::Cyan, "{}", abyss.image_url().unwrap());
	}

	ExitCode::SUCCESS
}