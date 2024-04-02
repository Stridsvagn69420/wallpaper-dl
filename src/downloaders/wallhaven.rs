use super::{quick_get, Downloader, DownloaderError, DownloaderResult};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct Wallhaven {
	path: String,
	id: String,
	category: String,
	purity: String,
	tags: Vec<Tag>,
}

#[derive(Deserialize, Clone)]
struct Tag {
	category: String,
	name: String,
	alias: String,
}

impl From<Tag> for Vec<String> {
	fn from(value: Tag) -> Vec<String> {
		let mut tags = vec![value.category, value.name];
		value.alias
			.split(", ")
			.for_each(|x| tags.push(x.to_string()));
		tags
	}
}

#[derive(Deserialize)]
struct WallhavenApi {
	data: Wallhaven,
}

#[async_trait]
impl Downloader for Wallhaven {
	async fn new(client: &Client, mut url: Url) -> DownloaderResult<Self> {
		let api_path = format!("/api/v1{}", url.path());
		url.set_path(&api_path);
		let api_res = quick_get(client, url).await?.json::<WallhavenApi>().await?;
		Ok(api_res.data)
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
	fn image_tags(&self) -> DownloaderResult<Vec<String>> {
		let mut tags = vec![self.category.clone(), self.purity.clone()];
		self.tags.clone()
			.into_iter()
			.flat_map(Into::<Vec<String>>::into)
			.for_each(|x| tags.push(x));
		Ok(tags)
	}
}