use super::{quick_get, Downloader, DownloaderError, DownloaderResult, ScraperWrapper, SelectAttr};
use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};
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
	async fn new(
		client: &Client,
		url: Url,
		id: String,
		service_css: &str,
		title_css: &str,
	) -> DownloaderResult<Self> {
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
		ScraperWrapper::image_url(&self.html, &self.download)
	}

	/// Image Title wrapper
	fn image_title(&self) -> DownloaderResult<String> {
		ScraperWrapper::image_title(&self.html, &self.title)
	}

	/// Image Tags wrapper
	/// 
	/// Image tags are the inner text of A-Tags inside a Div with class "well".
	fn image_tags(&self) -> DownloaderResult<Vec<String>> {
		Ok(self.html.select(&Selector::parse("div.well a")?)
			.flat_map(|x| x.text())
			.map(|y| y.trim().to_string())
			.collect())
	}
}

/// Wallpaper Abyss
///
/// Downloader designed for [Wallpaper Abyss](https://wall.alphacoders.com/)
pub struct WallpaperAbyss(Alphacoders);

#[async_trait]
impl Downloader for WallpaperAbyss {
	async fn new(client: &Client, url: Url) -> DownloaderResult<Self> {
		let id = match url.query() {
			Some(x) => x.replace("i=", ""),
			None => {
				return Err(DownloaderError::ParseError(
					"URL Query did not match pattern".to_string(),
				))
			}
		};
		let inner = Alphacoders::new(client, url, id, "wallpaper", "img#main-content").await?;
		Ok(Self(inner))
	}

	fn image_id(&self) -> &str {
		&self.0.id
	}
	fn image_url(&self) -> DownloaderResult<Url> {
		self.0.image_url()
	}
	fn image_title(&self) -> DownloaderResult<String> {
		self.0.image_title()
	}
	fn image_tags(&self) -> DownloaderResult<Vec<String>> {
		self.0.image_tags()
	}
}

/// Art Abyss
///
/// Downloader designed for [Art Abyss](https://art.alphacoders.com/)
pub struct ArtAbyss(Alphacoders);

#[async_trait]
impl Downloader for ArtAbyss {
	async fn new(client: &Client, url: Url) -> DownloaderResult<Self> {
		let id = url.path().replace("/arts/view/", "");
		let inner = Alphacoders::new(client, url, id, "art", "img.img-responsive").await?;
		Ok(Self(inner))
	}

	fn image_id(&self) -> &str {
		&self.0.id
	}
	fn image_url(&self) -> DownloaderResult<Url> {
		self.0.image_url()
	}
	fn image_title(&self) -> DownloaderResult<String> {
		self.0.image_title()
	}
	fn image_tags(&self) -> DownloaderResult<Vec<String>> {
		self.0.image_tags()
	}
}

/// Image Abyss
///
/// Downloader designed for [Image Abyss](https://pics.alphacoders.com/)
pub struct ImageAbyss(Alphacoders);

#[async_trait]
impl Downloader for ImageAbyss {
	async fn new(client: &Client, url: Url) -> DownloaderResult<Self> {
		let id = url.path().replace("/pictures/view/", "");
		let inner = Alphacoders::new(client, url, id, "picture", "img.img-responsive").await?;
		Ok(Self(inner))
	}

	fn image_id(&self) -> &str {
		&self.0.id
	}
	fn image_url(&self) -> DownloaderResult<Url> {
		self.0.image_url()
	}
	fn image_title(&self) -> DownloaderResult<String> {
		self.0.image_title()
	}
	fn image_tags(&self) -> DownloaderResult<Vec<String>> {
		self.0.image_tags()
	}
}