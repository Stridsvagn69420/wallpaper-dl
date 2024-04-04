use std::env;
use std::process::Command;

// TODO: Offload this to apputils
fn main() {
	// ----- Target Triple -----
	println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());

	// ----- Rust Version -----
	println!("cargo:rustc-env=RUST_VERSION={}", version("rustc"));
}

fn version(name: &str) -> String {
	let output = Command::new(name).arg("--version").output().unwrap();
	let raw = String::from_utf8(output.stdout).unwrap();
	raw.replace(&format!("{} ", name), "")
}