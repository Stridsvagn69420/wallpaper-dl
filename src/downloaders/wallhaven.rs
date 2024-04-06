use super::{quick_get, Downloader, DownloaderError, DownloaderResult, Urls};
use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;

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

/// Wallhaven
/// 
/// Downloader designed for [Wallhaven](https://wallhaven.cc/)
#[derive(Deserialize)]
pub struct Wallhaven {
	path: String,
	id: String,
	category: String,
	purity: String,
	tags: Vec<Tag>,
}

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
		let url = Url::parse(&self.path)?;
		Ok(url.into())
	}
	fn image_title(&self) -> DownloaderResult<String> {
		Err(DownloaderError::Other)
	}
	fn image_tags(&self) -> DownloaderResult<Vec<String>> {
		let mut tags = vec![self.category.clone(), self.purity.clone()];
		self.tags.clone()
			.into_iter()
			.flat_map(Into::<Vec<String>>::into)
			.filter(|x| !x.is_empty())
			.for_each(|y| tags.push(y));
		Ok(tags)
	}
}