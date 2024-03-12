use std::error::Error;
use apputils::Colors;
use apputils::{paint, paintln};
use reqwest::Url;
use reqwest;
use scraper::{Html, Selector};

pub(crate) fn wallhaven_test(link: &str) -> Result<(), Box<dyn Error>> {
	// Parse UR
	let url = Url::parse(link)?;

	// Check hostname
	let Some(host) = url.host_str() else {
		paintln!(Colors::Red, "The given URL does not have a host field!");
		return Ok(());
	};
	paintln!(Colors::Yellow, "Your URL's host is: {}", host);

	// Make HTTP Request and extract HTML document
	paintln!(Colors::Cyan, "Making HTTP request to {}...", url);
	let response = reqwest::blocking::get(url)?.error_for_status()?.text()?;

	// Parse HTML document and get Image Tag
	let html = Html::parse_document(&response);
	let selector = Selector::parse("img#wallpaper")?;
	let html_tags = html.select(&selector);

	if html_tags.clone().count() > 0 {
		paintln!(Colors::Cyan, "Found HTML elements!");
	}
	for x in html_tags {
		paint!(Colors::Red, "Raw HTML: ");
		println!("{:?}", x.html());

		paint!(Colors::Red, "Image source: ");
		println!("{}", x.value().attr("data-cfsrc").unwrap_or("None"));

		paint!(Colors::Red, "Alt text: ");
		println!("{}", x.value().attr("alt").unwrap_or("None"));

		println!();
	}

	Ok(())
}