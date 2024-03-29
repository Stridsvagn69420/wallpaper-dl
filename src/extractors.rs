//! Extractors
//! 
//! The core part of extracting data from a website.
//! Methods can differ, e.g. webscraping, API, etc., so this submodule neatly contains them.

use scraper::{Html, Selector};
use scraper::error::SelectorErrorKind;

pub struct SelectAttr {
	select: Selector,
	attr: String
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
pub fn src_tag_attr<'a>(html: &'a Html, elem: &SelectAttr) -> Option<&'a str> {
	html.select(&elem.select).next()
	.and_then(|x| x.attr(&elem.attr))
}

/// Source-Tag-Inner Extractor
/// 
/// The simplest of them all 2.0.
/// It's essentially [src_tag_attr], but it returns the inner HTML
/// and not the value of an attribute.
pub fn src_tag_inner(html: &Html, elem: &Selector) -> Option<String>{
	let tag = html.select(elem).next()?;
	Some(tag.inner_html())
}