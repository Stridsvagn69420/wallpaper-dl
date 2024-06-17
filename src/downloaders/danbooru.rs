use super::{quick_get, Downloader, DownloaderError, DownloaderResult, ScraperWrapper, SelectAttr, Urls};
use reqwest::blocking::Client;
use scraper::Html;
use url::Url;

pub struct Danbooru;

#[allow(refining_impl_trait)]
impl Downloader for Danbooru {
	fn new(client: &Client, url: Url) -> DownloaderResult<Self> {
		todo!()
	}

	fn image_id(&self) -> &str {
		todo!()
	}

	fn image_url(&self) -> DownloaderResult<Urls> {
		todo!()
	}
}