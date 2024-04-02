use std::env;
use std::process::ExitCode;
use apputils::paint;
use apputils::Colors;
use apputils::paintln;
use reqwest::Client;
use url::Url;

mod downloaders;
use downloaders::DownloaderError;

mod config;
mod meta;
use meta::USER_AGENT;

#[tokio::main]
async fn main() -> ExitCode {
	let args: Vec<Url> = env::args().skip(1).filter_map(|x| Url::parse(&x).ok()).collect();
	if args.is_empty() {
		paintln!(Colors::Red, "No URLs provided!");
		return ExitCode::FAILURE;
	}

	let Ok(client) = Client::builder().user_agent(USER_AGENT).build() else {
		paintln!(Colors::Red, "Failed to build reqwest client!");
		return ExitCode::FAILURE;
	};

	// Temporary proof-of-concept and blockign iterator
	for url in args.into_iter() {
		print!("URL: ");
		paintln!(Colors::Green, "{url}");

		let walldl = match downloaders::from_url(&client, url).await {
			Ok(dl) => dl,
			Err(err) => {
				let msg = match err {
					DownloaderError::Other => "Website not supported".to_string(),
					_ => format!("An error occured during the initial request: {:?}", err)
				};
				paintln!(Colors::Red, "{}", msg);
				continue;
			}
		};

		print!("Title: ");
		match walldl.image_title() {
			Ok(title ) => paintln!(Colors::Cyan, "{}", title),
			Err(_) => paintln!(Colors::Red, "Not found")
		}

		print!("ID: ");
		paintln!(Colors::Cyan, "{}", walldl.image_id());

		print!("URL: ");
		match walldl.image_url() {
			Ok(url ) => paintln!(Colors::Cyan, "{}", url),
			Err(_) => paintln!(Colors::Red, "Could not be retrieved!")
		}

		print!("Tags: ");
		match walldl.image_tags() {
			Ok(mut tags) => {
				tags.sort_unstable();
				tags.dedup();
				let mut it = tags.into_iter().peekable();
				while let Some(tag) = it.next() {
					paint!(Colors::Cyan, "{tag}");
					if it.peek().is_some() {
						print!(", ");
					}				
				}
				println!("");
			},
			Err(_) => paintln!(Colors::Red, "Could not be retrieved!")
		}

		println!("\n{}\n", "-".repeat(50));
	}

	ExitCode::SUCCESS
}
