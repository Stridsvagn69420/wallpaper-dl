use super::{quick_get, Downloader, DownloaderError, DownloaderResult, ScraperWrapper, SelectAttr, Urls};
use reqwest::blocking::Client;
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
	/// `client`: The borrowed Reqwest [Client].
	/// `url`: The post url
	/// `id`: The image ID parsed from the URL.  
	/// `service_css`: A keyword in the CSS rules of Alphacoders to find the download button.  
	/// `title_css`: The CSS selector for the HTML element that contains the title field.
	fn new(
		client: &Client,
		url: Url,
		delay: u64,
		id: String,
		service_css: &str,
		title_css: &str	
	) -> DownloaderResult<Self> {
		let html = quick_get(client, url, delay)?.text()?;
		let download_css = format!("a#{}_{}_download_button", service_css, id);
		let download = SelectAttr::parse(&download_css, "href")?;
		Ok(Self {
			html: Html::parse_document(&html),
			id,
			download,
			title: SelectAttr::parse(title_css, "title")?
		})
	}

	/// Image URL wrapper
	fn image_url(&self) -> DownloaderResult<Urls> {
		let url = ScraperWrapper::image_url(&self.html, &self.download)?;
		Ok(Urls::Single(url))
	}

	/// Image Title wrapper
	fn image_title(&self) -> DownloaderResult<String> {
		ScraperWrapper::image_title(&self.html, &self.title)
	}
}

/// Wallpaper Abyss
///
/// Downloader designed for [Wallpaper Abyss](https://wall.alphacoders.com/)
pub struct WallpaperAbyss(Alphacoders);

impl Downloader for WallpaperAbyss {
	fn new(client: &Client, url: Url, delay: u64) -> DownloaderResult<Self> {
		let id = match url.query() {
			Some(x) => x.replace("i=", ""),
			None => return Err(DownloaderError::ParseError("URL Query did not match pattern".to_string()))
		};
		let inner = Alphacoders::new(client, url, delay, id, "wallpaper", "img#main-content")?;
		Ok(Self(inner))
	}

	fn image_id(&self) -> &str {
		&self.0.id
	}
	fn image_url(&self) -> DownloaderResult<Urls> {
		self.0.image_url()
	}
	fn image_title(&self) -> DownloaderResult<String> {
		self.0.image_title()
	}
}

/// Art Abyss
///
/// Downloader designed for [Art Abyss](https://art.alphacoders.com/)
pub struct ArtAbyss(Alphacoders);

impl Downloader for ArtAbyss {
	fn new(client: &Client, url: Url, delay: u64) -> DownloaderResult<Self> {
		let id = url.path().replace("/arts/view/", "");
		let inner = Alphacoders::new(client, url, delay, id, "art", "img.img-responsive")?;
		Ok(Self(inner))
	}

	fn image_id(&self) -> &str {
		&self.0.id
	}
	fn image_url(&self) -> DownloaderResult<Urls> {
		self.0.image_url()
	}
	fn image_title(&self) -> DownloaderResult<String> {
		self.0.image_title()
	}
}

/// Image Abyss
///
/// Downloader designed for [Image Abyss](https://pics.alphacoders.com/)
pub struct ImageAbyss(Alphacoders);

impl Downloader for ImageAbyss {
	fn new(client: &Client, url: Url, delay: u64) -> DownloaderResult<Self> {
		let id = url.path().replace("/pictures/view/", "");
		let inner = Alphacoders::new(client, url, delay, id, "picture", "img.img-responsive")?;
		Ok(Self(inner))
	}

	fn image_id(&self) -> &str {
		&self.0.id
	}
	fn image_url(&self) -> DownloaderResult<Urls> {
		self.0.image_url()
	}
	fn image_title(&self) -> DownloaderResult<String> {
		self.0.image_title()
	}
}