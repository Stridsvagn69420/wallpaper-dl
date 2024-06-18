use super::{quick_get, Downloader, DownloaderResult, Urls};
use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
struct WallhavenApi {
	data: Wallhaven,
}

/// Wallhaven
/// 
/// Downloader designed for [Wallhaven](https://wallhaven.cc/)
#[derive(Deserialize)]
pub struct Wallhaven {
	path: Url,
	id: String
}

#[allow(refining_impl_trait)]
impl Downloader for Wallhaven {
	fn new(client: &Client, mut url: Url) -> DownloaderResult<Self> {
		let api_path = format!("/api/v1{}", url.path());
		url.set_path(&api_path);
		let api_res = quick_get(client, url)?.json::<WallhavenApi>()?;
		Ok(api_res.data)
	}
	fn image_id(&self) -> &str {
		&self.id
	}
	fn image_url(&self) -> DownloaderResult<Urls> {
		Ok(self.path.clone().into())
	}
}