use super::{Webscraper, DownloaderResult};
use crate::extractors::SelectAttr;
use scraper::Html;

/// ArtStation
/// 
/// Scraper designed for [ArtStation](https://www.artstation.com/)
pub struct ArtStation {
	html: Html,
	id: String,
	download: SelectAttr,
	title: SelectAttr
}

impl Webscraper for ArtStation {
	fn new(html: &str, url: &url::Url) -> DownloaderResult<Self> {
		let id = url.path().replace("/artwork/", "");
		Ok(Self {
			html: Html::parse_document(html),
			id,
			download: SelectAttr::parse("meta[name=\"image\"]", "content")?,
			title: SelectAttr::parse("meta[name=\"twitter:title\"]", "content")?
		})
	}

	fn source_html(&self) -> &Html {
		&self.html
	}

	fn selector_download(&self) -> &SelectAttr {
		&self.download
	}

	fn selector_title(&self) -> &SelectAttr {
		&self.title
	}

	fn image_id(&self) -> &str {
		&self.id
	}
}