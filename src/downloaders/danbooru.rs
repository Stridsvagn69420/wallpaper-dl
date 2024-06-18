use super::{quick_get, Downloader, DownloaderResult, Urls};
use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct DanbooruApi {
	id: u32,
	file_url: Url
}

pub struct Danbooru {
	id: String,
	file_url: Url
}

impl From<DanbooruApi> for Danbooru {
	fn from(value: DanbooruApi) -> Danbooru {
		Danbooru {
			file_url: value.file_url,
			id: value.id.to_string()
		}
	}
}

#[allow(refining_impl_trait)]
impl Downloader for Danbooru {
	fn new(client: &Client, mut url: Url) -> DownloaderResult<Self> {
		// Append .json extension
		let newpath = format!("{}.json", url.path());
		url.set_path(&newpath);

		// Make API Request
		let api_res = quick_get(client, url)?.json::<DanbooruApi>()?;
		Ok(api_res.into())
	}
	fn image_id(&self) -> &str {
		&self.id
	}
	fn image_url(&self) -> DownloaderResult<Urls> {
		Ok(self.file_url.clone().into())
	}
}