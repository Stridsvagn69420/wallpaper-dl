use apputils::{paint, paintln, Colors};
use reqwest::blocking::Client;
use std::env;
use std::process::ExitCode;
use url::Url;

mod downloaders;
use downloaders::{Urls, WallpaperMeta};

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

	// Parse first argument
	let Some(cmd) = args.first() else {
		return Info::help(true);
	};

	// Create HTTP Client
	let Ok(client) = Client::builder().user_agent(USER_AGENT).build() else {
		paintln!(Colors::RedBold, "Failed to initialize HTTP-Client!");
		return ExitCode::FAILURE;
	};
	ExitCode::SUCCESS
}