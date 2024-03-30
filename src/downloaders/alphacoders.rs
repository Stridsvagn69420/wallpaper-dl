use super::{ImageDownloader, DownloaderError, DownloaderResult, quick_get};
use crate::webscraper::{SelectAttr, Wrapper};
use reqwest::Client;
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
	async fn new(client: &Client, url: Url, id: String, service_css: &str, title_css: &str) -> DownloaderResult<Self> {
		let html = quick_get(client, url).await?.text().await?;
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
	fn image_url(&self) -> DownloaderResult<Url> {
		Wrapper::image_url(&self.html, &self.download)
	}

	/// Image Title wrapper
	fn image_title(&self) -> DownloaderResult<String> {
		Wrapper::image_title(&self.html, &self.title)
	}
}

/// Wallpaper Abyss
/// 
/// Downloader designed for [Wallpaper Abyss](https://wall.alphacoders.com/)
pub struct WallAbyss {
	inner: Alphacoders
}

impl ImageDownloader for WallAbyss {
	async fn new(client: &Client, url: Url) -> DownloaderResult<Self> {
		let id = match url.query() {
			Some(x) => x.replace("i=", ""),
			None => return Err(DownloaderError::ParseError("URL Query did not match pattern".to_string()))
		};
		let inner = Alphacoders::new(client, url, id, "wallpaper", "img#main-content").await?;
		Ok(Self { inner })
	}

	fn image_id(&self) -> &str {
		&self.inner.id
	}
	async fn image_url(&self) -> DownloaderResult<Url> {
		self.inner.image_url()
	}
	async fn image_title(&self) -> DownloaderResult<String> {
		self.inner.image_title()
	}
}

/// Art Abyss
/// 
/// Downloader designed for [Art Abyss](https://art.alphacoders.com/)
pub struct ArtAbyss {
	inner: Alphacoders
}

impl ImageDownloader for ArtAbyss {
	async fn new(client: &Client, url: Url) -> DownloaderResult<Self> {
		let id = url.path().replace("/arts/view/", "");
		let inner = Alphacoders::new(client, url, id, "art", "img.img-responsive").await?;
		Ok(Self { inner })
	}

	fn image_id(&self) -> &str {
		&self.inner.id
	}
	async fn image_url(&self) -> DownloaderResult<Url> {
		self.inner.image_url()
	}
	async fn image_title(&self) -> DownloaderResult<String> {
		self.inner.image_title()
	}
}

/// Image Abyss
/// 
/// Downloader designed for [Image Abyss](https://pics.alphacoders.com/)
pub struct ImageAbyss {
	inner: Alphacoders
}

impl ImageDownloader for ImageAbyss {
	async fn new(client: &Client, url: Url) -> DownloaderResult<Self> {
		let id = url.path().replace("/pictures/view/", "");
		let inner = Alphacoders::new(client, url, id, "picture", "img.img-responsive").await?;
		Ok(Self { inner })
	}

	fn image_id(&self) -> &str {
		&self.inner.id
	}
	async fn image_url(&self) -> DownloaderResult<Url> {
		self.inner.image_url()
	}
	async fn image_title(&self) -> DownloaderResult<String> {
		self.inner.image_title()
	}
}