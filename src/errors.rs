use apputils::{Colors, paint, paintln};
use std::fmt::Display;
use std::io;
use std::process::ExitCode;

pub struct MainErr;
impl MainErr {
	pub fn db_load() -> ExitCode {
		paintln!(Colors::RedBold, "Failed to read wallpaper database!");
		ExitCode::FAILURE
	}

	pub fn db_load_notfound() -> ExitCode {
		paintln!(Colors::YellowBold, "Wallpaper database not found!");
		ExitCode::FAILURE
	}

	pub fn db_not_found(c: Colors, t: &str) -> ExitCode {
		paintln!(c, "{t} not found in database!");
		ExitCode::FAILURE
	}

	pub fn db_param_not_found(ctyp: Colors, cdata: Colors, typ: &str, payload: impl Display) -> ExitCode {
		paint!(ctyp, "{typ} ");
		paint!(cdata, "{payload}");
		paintln!(ctyp, " not found in database!");
		ExitCode::FAILURE
	}

	pub fn cfg_load(err: io::Error) -> ExitCode {
		match err.kind() {
			io::ErrorKind::NotFound => paintln!(Colors::YellowBold, "Config not found!"),
			_ => return key_value(Colors::RedBold, Colors::Red, "Failed to read config file", err)
		}
		ExitCode::FAILURE
	}

	pub fn cfg_save(ctext: Colors, cerr: Colors, err: io::Error) -> ExitCode {
		key_value(ctext, cerr, "Failed to save configuration", err)
	}

	pub fn cfg_wallpaper() -> ExitCode {
		paintln!(Colors::Yellow, "No wallpaper was set");
		ExitCode::FAILURE
	}
}

/// Key-Value error formatting
/// 
/// Prints out an error in a Key-Value-like style.
fn key_value(ctext: Colors, cerr: Colors, text: &str, err: impl Display) -> ExitCode {
	paint!(ctext, "{text}: ");
	paintln!(cerr, "{err}");
	ExitCode::FAILURE
}