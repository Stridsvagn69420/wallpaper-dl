//! Crate Metadata
//! 
//! Submodule for metadata about this crate.

use apputils::{paint, paintln, Colors};
use std::env::consts::{ARCH, OS};
use std::process::ExitCode;

// App Metadata
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DESC: &str = env!("CARGO_PKG_DESCRIPTION");

// Config Constants
pub const CONFIG_FILE: &str = "config.toml";
pub const WALLPAPERS_FILE: &str = "wallpapers.toml";

// HTTP Constants
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), '/', env!("CARGO_PKG_VERSION"));

pub struct Info;
impl Info {
	pub fn version() -> ExitCode {
		println!("{APP_NAME} {APP_VERSION} {OS}/{ARCH}");
		ExitCode::SUCCESS
	}

	pub fn help(failed: bool) -> ExitCode {
		// ---- USAGE ----
		paintln!(Colors::RedBold, "Usage:");
		paint!(Colors::Red, "  {} <URL[]> ", APP_NAME);
		println!("{}", APP_DESC);
		

		// ---- SUBCOMMANDS ----
		println!();
		paintln!(Colors::MagentaBold, "Subcommands:");

		// wallpaper-dl current
		paint!(Colors::Magenta, "  current        ");
		println!("Display path of current wallpaper");

		// wallpaper-dl current <URL>
		paint!(Colors::Magenta, "  current <URL>  ");
		println!("Set current wallpaper by URL and download if it's missing");

		// wallpaper-dl current <Path>
		paint!(Colors::Magenta, "  current <Path> ");
		println!("Set current wallpaper by filepath relative to wallpaper folder root");

		// ---- FLAGS ----
		println!();
		paintln!(Colors::CyanBold, "Flags:");

		// wallpaper-dl --help
		paint!(Colors::Cyan, "  -h, --help     ");
		println!("Print this help message");

		// wallpaper-dl --version
		paint!(Colors::Cyan, "  -V, --version  ");
		println!("Print app version");

		ExitCode::from(Into::<u8>::into(failed))
	}
}