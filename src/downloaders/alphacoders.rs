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
	download: SelectAttr
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
		id: String,
		service_css: &str
	) -> DownloaderResult<Self> {
		let html = quick_get(client, url)?.text()?;
		let download_css = format!("a#{}_{}_download_button", service_css, id);
		let download = SelectAttr::parse(&download_css, "href")?;
		Ok(Self {
			html: Html::parse_document(&html),
			id,
			download
		})
	}

	/// Image URL wrapper
	fn image_url(&self) -> DownloaderResult<Urls> {
		let url = ScraperWrapper::image_url(&self.html, &self.download)?;
		Ok(Urls::Single(url))
	}
}

/// Wallpaper Abyss
///
/// Downloader designed for [Wallpaper Abyss](https://wall.alphacoders.com/)
pub struct WallpaperAbyss(Alphacoders);

#[allow(refining_impl_trait)]
impl Downloader for WallpaperAbyss {
	fn new(client: &Client, url: Url) -> DownloaderResult<Self> {
		let id = match url.query() {
			Some(x) => x.replace("i=", ""),
			None => return Err(DownloaderError::ParseError("URL Query did not match pattern".to_string()))
		};
		let inner = Alphacoders::new(client, url, id, "wallpaper")?;
		Ok(Self(inner))
	}

	fn image_id(&self) -> &str {
		&self.0.id
	}
	fn image_url(&self) -> DownloaderResult<Urls> {
		self.0.image_url()
	}
}

/// Art Abyss
///
/// Downloader designed for [Art Abyss](https://art.alphacoders.com/)
pub struct ArtAbyss(Alphacoders);

impl Downloader for ArtAbyss {
	#[allow(refining_impl_trait)]
	fn new(client: &Client, url: Url) -> DownloaderResult<Self> {
		let id = url.path().replace("/arts/view/", "");
		let inner = Alphacoders::new(client, url, id, "art")?;
		Ok(Self(inner))
	}

	fn image_id(&self) -> &str {
		&self.0.id
	}
	fn image_url(&self) -> DownloaderResult<Urls> {
		self.0.image_url()
	}
}

/// Image Abyss
///
/// Downloader designed for [Image Abyss](https://pics.alphacoders.com/)
pub struct ImageAbyss(Alphacoders);

impl Downloader for ImageAbyss {
	#[allow(refining_impl_trait)]
	fn new(client: &Client, url: Url) -> DownloaderResult<Self> {
		let id = url.path().replace("/pictures/view/", "");
		let inner = Alphacoders::new(client, url, id, "picture")?;
		Ok(Self(inner))
	}

	fn image_id(&self) -> &str {
		&self.0.id
	}
	fn image_url(&self) -> DownloaderResult<Urls> {
		self.0.image_url()
	}
}