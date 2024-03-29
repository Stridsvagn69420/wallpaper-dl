use crate::extractors::{src_tag_inner, src_tag_attr, SelectAttr};
use scraper::{Html, Selector};
use reqwest::Url;

pub struct WallpaperAbyss {
	html: Html,
	id: String,
	download: SelectAttr,
	title: Selector,
}

impl WallpaperAbyss {
	pub fn new(html: &str, url: &Url) -> Option<Self> {
		let id = url.query()?.replace("i=", "");
		let css_query = format!("a#wallpaper_{}_download_button", id);
		let download = SelectAttr::parse(&css_query, "href").ok()?;

		Some(Self {
    		download,
			id,
    		title: Selector::parse("title").ok()?,
			html: Html::parse_document(html),
		})
	}

	pub fn image_url(&self) -> Option<Url> {
		let link = src_tag_attr(&self.html, &self.download)?;
		Url::parse(link).ok()
	}

	pub fn image_title(&self) -> Option<String> {
		src_tag_inner(&self.html, &self.title)
	}

	pub fn image_id(&self) -> &str {
		&self.id
	}
}