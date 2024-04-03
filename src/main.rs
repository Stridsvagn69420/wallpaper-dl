use apputils::{Colors, paint, paintln};
use reqwest::Client;
use std::env;
use std::process::ExitCode;
use url::Url;

mod downloaders;
use downloaders::{DownloaderError, Urls};

mod config;
mod meta;
use meta::USER_AGENT;

#[tokio::main]
async fn main() -> ExitCode {
	let args: Vec<Url> = env::args()
		.skip(1)
		.filter_map(|x| Url::parse(&x).ok())
		.collect();
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
				Urls::Multi(urls) => pretty_print(Colors::Red, urls)
			}
		}

		print!("Tags: ");
		match walldl.image_tags() {
			Err(_) => paintln!(Colors::Red, "Could not be retrieved!"),
			Ok(mut tags) => {
				tags.sort_unstable();
				tags.dedup();
				pretty_print(Colors::Blue, tags);
			}
		}

		println!("\n{}\n", "-".repeat(50));
	}

	ExitCode::SUCCESS
}

fn pretty_print<I: IntoIterator>(c: Colors, vec: I)
where
	<I as IntoIterator>::Item: std::fmt::Display,
{
	let mut iter = vec.into_iter().peekable();
	while let Some(item) = iter.next() {
		paint!(c, "{item}");
		if iter.peek().is_some() {
			print!(", ");
		}
	}
	println!();
}