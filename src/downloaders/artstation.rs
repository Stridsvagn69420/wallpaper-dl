use super::{quick_get, Downloader, DownloaderResult, Urls};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

/// ArtStation
/// 
/// Downloader designed for [ArtStation](https://www.artstation.com/)
#[derive(Deserialize)]
pub struct ArtStation {
	hash_id: String,
	title: String,
	assets: Vec<Asset>,
	tags: Vec<String>
}

#[async_trait]
impl Downloader for ArtStation {
	async fn new(client: &Client, mut url: Url) -> DownloaderResult<Self> {
		let id_path = url.path().replace("/artwork/", "/projects/");
		let api_path = format!("{}.json", id_path);
		url.set_path(&api_path);
		Ok(quick_get(client, url).await?.json::<Self>().await?)
	}

	fn image_id(&self) -> &str {
		&self.hash_id
	}
	fn image_url(&self) -> DownloaderResult<Urls> {
		let urls = self.assets
			.iter()
			.filter_map(|x| {
				if x.asset_type != "image" {
					return None;
				}
				// ArtStation uses "4k" for the original size.
				let url_hires = x.image_url.replace("/large/", "/4k/");
				Url::parse(&url_hires).ok()
			})
			.collect();
		Ok(Urls::Multi(urls))
	}
	fn image_title(&self) -> DownloaderResult<String> {
		Ok(self.title.clone())
	}
	fn image_tags(&self) -> DownloaderResult<Vec<String>> {
		Ok(self.tags.clone())
	}
}
/// ArtStation Post Asset
#[derive(Deserialize)]
struct Asset {
	image_url: String,
	asset_type: String
}