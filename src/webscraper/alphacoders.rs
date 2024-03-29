use super::{Webscraper, ScraperError, ScraperResult};
use crate::extractors::SelectAttr;
use scraper::Html;
use url::Url;

/// Alphacoders Core
/// 
/// The core parts of every Alphacoders-based scraper.
struct Alphacoders {
	html: Html,
	id: String,
	download: SelectAttr,
	title: SelectAttr
}

impl Alphacoders {
	/// New Alphacoders Core
	/// 
	/// This needs some documentation.  
	/// `html`: The raw HTML source code that will be parsed.  
	/// `id`: The image ID parsed from the URL.  
	/// `service_css`: A keyword in the CSS rules of Alphacoders to find the download button.  
	/// `title_css`: The CSS selector for the HTML element that contains the title field.
	fn new(html: &str, id: String, service_css: &str, title_css: &str) -> ScraperResult<Self> {
		let download_css = format!("a#{}_{}_download_button", service_css, id);
		let download = SelectAttr::parse(&download_css, "href")?;
		Ok(Self {
			html: Html::parse_document(html),
			id,
			download,
			title: SelectAttr::parse(title_css, "title")?
		})
	}
}

/// Wallpaper Abyss
/// 
/// Scraper designed for [Wallpaper Abyss](https://wall.alphacoders.com/)
pub struct WallAbyss {
	inner: Alphacoders
}

impl Webscraper for WallAbyss {
	fn new(html: &str, url: &Url) -> ScraperResult<Self> {
		let Some(query) = url.query() else {
			return Err(ScraperError::ParseError("URL Path did not match pattern".to_string()));
		};
		let inner = Alphacoders::new(html, query.replace("i=", ""), "wallpaper", "img#main-content")?;
		Ok(Self { inner })
	}

	fn source_html(&self) -> &Html {
		&self.inner.html
	}
	fn selector_download(&self) -> &SelectAttr {
		&self.inner.download
	}
	fn selector_title(&self) -> &SelectAttr {
		&self.inner.title
	}
	fn image_id(&self) -> &str {
		&self.inner.id
	}
}

/// Art Abyss
/// 
/// Scraper designed for [Art Abyss](https://art.alphacoders.com/)
pub struct ArtAbyss {
	inner: Alphacoders
}

impl Webscraper for ArtAbyss {
	fn new(html: &str, url: &Url) -> ScraperResult<Self> {
		let id = url.path().replace("/arts/view/", "");
		let inner = Alphacoders::new(html, id, "art", "img.img-responsive")?;
		Ok(Self { inner })
	}

	fn source_html(&self) -> &Html {
		&self.inner.html
	}
	fn selector_download(&self) -> &SelectAttr {
		&self.inner.download
	}
	fn selector_title(&self) -> &SelectAttr {
		&self.inner.title
	}
	fn image_id(&self) -> &str {
		&self.inner.id
	}
}

/// Image Abyss
/// 
/// Scraper designed for [Image Abyss](https://pics.alphacoders.com/)
pub struct ImageAbyss {
	inner: Alphacoders
}

impl Webscraper for ImageAbyss {
	fn new(html: &str, url: &Url) -> ScraperResult<Self> {
		let id = url.path().replace("/pictures/view/", "");
		let inner = Alphacoders::new(html, id, "picture", "img.img-responsive")?;
		Ok(Self { inner })
	}

	fn source_html(&self) -> &Html {
		&self.inner.html
	}
	fn selector_download(&self) -> &SelectAttr {
		&self.inner.download
	}
	fn selector_title(&self) -> &SelectAttr {
		&self.inner.title
	}
	fn image_id(&self) -> &str {
		&self.inner.id
	}
}