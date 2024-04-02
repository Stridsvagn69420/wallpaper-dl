use super::{Downloader, DownloaderResult, quick_get, SelectAttr, ScraperWrapper};
use async_trait::async_trait;
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

#[async_trait]
impl Downloader for ArtStation {
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
	fn image_url(&self) -> DownloaderResult<Url> {
		ScraperWrapper::image_url(&self.html, &self.download)
	}
	fn image_title(&self) -> DownloaderResult<String> {
		ScraperWrapper::image_title(&self.html, &self.title)
	}
	fn image_tags(&self) -> DownloaderResult<Vec<String>> {
		todo!() // TODO: Find a way to read tags
	}
}