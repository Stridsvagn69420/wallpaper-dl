use std::error::Error;
use std::env;
use apputils::Colors;
use apputils::paintln;

mod test;

fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		paintln!(Colors::Red, "No URLs provided!");
		return Ok(());
	}
	for url in args.into_iter().skip(1) {
		paintln!(Colors::Magenta, "URL: {}", url);
		let _ = test::wallhaven_test(&url);
	}
	Ok(())
}