use super::{Downloader, DownloaderResult, DownloaderError, quick_get};
use async_trait::async_trait;
use url::Url;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Wallhaven {
	path: String,
	id: String
}

#[async_trait]
impl Downloader for Wallhaven {
	async fn new(client: &Client, url: Url) -> DownloaderResult<Self> {
		let apires = quick_get(client, url).await?
			.json::<WallhavenApi>().await?;
		Ok(apires.data)
	}

	fn image_id(&self) -> &str {
		&self.id
	}
	fn image_url(&self) -> DownloaderResult<Url> {
		Ok(Url::parse(&self.path)?)
	}
	fn image_title(&self) -> DownloaderResult<String> {
		Err(DownloaderError::Other)
	}
}

#[derive(Deserialize)]
struct WallhavenApi {
	data: Wallhaven
}