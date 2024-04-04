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
const APP_REPO: &str = env!("CARGO_PKG_REPOSITORY");
const APP_LICENSE: &str = env!("CARGO_PKG_LICENSE");

// build.rs Metadata
const RUST_TARGET: &str = env!("TARGET");
const RUST_VERSION: &str = env!("RUST_VERSION");

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
		let code = if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS };

		eprintln!("Work in progess...");

		code
	}
}