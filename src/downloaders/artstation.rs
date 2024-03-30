use super::{ImageDownloader, DownloaderResult, quick_get};
use crate::webscraper::{SelectAttr, Wrapper};
use reqwest::Client;
use scraper::Html;
use url::Url;

/// ArtStation
/// 
/// Scraper designed for [ArtStation](https://www.artstation.com/)
pub struct ArtStation {
	html: Html,
	id: String,
	download: SelectAttr,
	title: SelectAttr
}

impl ImageDownloader for ArtStation {
	async fn new(client: &Client, url: Url) -> DownloaderResult<Self> {
		let id = url.path().replace("/artwork/", "");
		let html = quick_get(client, url.to_owned()).await?.text().await?;
		Ok(Self {
			html: Html::parse_document(&html),
			id,
			download: SelectAttr::parse("meta[name=\"image\"]", "content")?,
			title: SelectAttr::parse("meta[name=\"twitter:title\"]", "content")?
		})
	}

	fn image_id(&self) -> &str {
		&self.id
	}
	async fn image_url(&self) -> DownloaderResult<Url> {
		Wrapper::image_url(&self.html, &self.download)
	}
	async fn image_title(&self) -> DownloaderResult<String> {
		Wrapper::image_title(&self.html, &self.title)
	}
}