use std::env;
use std::process::ExitCode;
use apputils::Colors;
use apputils::paintln;
use reqwest::Client;
use url::Url;

mod downloaders;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), '/', env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() -> ExitCode {
	let args: Vec<Url> = env::args().skip(1).filter_map(|x| Url::parse(&x).ok()).collect();
	if args.len() < 1 {
		paintln!(Colors::Red, "No URLs provided!");
		return ExitCode::FAILURE;
	}

	let Ok(client) = Client::builder().user_agent(USER_AGENT).build() else {
		paintln!(Colors::Red, "Failed to build reqwest client!");
		return ExitCode::FAILURE;
	};

	// Temporary test for Alphacoders
	for url in args.into_iter() {
		println!();
		paintln!(Colors::Green, "URL: {}", url);

		paintln!(Colors::Yellow, "Downloading...");
		let Ok(abyss) = downloaders::from_url(&client, url).await else {
			paintln!(Colors::Red, "Failed to download image!");
			continue;
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
