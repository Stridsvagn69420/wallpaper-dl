//! Webscraper helpers
//! 
//! Helper functions and shared code for webscraping-based downloaders.

use url::Url;
use scraper::{Html, Selector};
use scraper::error::SelectorErrorKind;
use crate::downloaders::{DownloaderResult, DownloaderError};

pub struct SelectAttr {
	pub select: Selector,
	pub attr: String
}

impl SelectAttr {
	pub fn parse<'a>(css: &'a str, attr: &'a str) -> Result<Self, SelectorErrorKind<'a>> {
		Ok(Self {
			select: Selector::parse(css)?,
			attr: attr.to_string()
		})
	}
}

/// Source-Tag-Attribute Extractor
/// 
/// The simplest of them all!
/// It uses the [Html] source to find a Tag that matches the [Selector].
/// Then it reads the provided `attr` and returns it as a &[str].
/// 
/// Parsing of the main HTML-Document and creating the selector must be done ahead.
/// This removes the need for parsing the CSS Selector and allows you to re-use the [Html].
fn src_tag_attr<'a>(html: &'a Html, elem: &SelectAttr) -> Option<&'a str> {
	html.select(&elem.select).next()
	.and_then(|x| x.attr(&elem.attr))
}

/// Attribute from Element wrapper
/// 
/// Wrapper around [src_tag_attr] to convert the [Option] to a [DownloaderResult].
fn attr_from_element<'a>(html: &'a Html, elem: &SelectAttr) -> DownloaderResult<&'a str> {
	match src_tag_attr(html, elem) {
		None => Err(DownloaderError::ParseError("HTML element or attribute not found.".to_string())),
		Some(x) => Ok(x)
	}
}

/// Webscraper extractor wrappers
/// 
/// Basically just a collection of functions that are re-used by webscraper downloaders.
pub struct Wrapper;

impl Wrapper {
	/// Image URL wrapper
	/// 
	/// Extract value from element and parse it as a [Url].
	pub fn image_url(html: &Html, select: &SelectAttr) -> DownloaderResult<Url> {
		let link = attr_from_element(html, select)?;
		Ok(Url::parse(link)?)
	}

	/// Image Title wrapper
	/// 
	/// Extract value from element and convert it to a [String].
	pub fn image_title(html: &Html, select: &SelectAttr) -> DownloaderResult<String> {
		let title = attr_from_element(html, select)?;
		Ok(title.to_string())
	}
}