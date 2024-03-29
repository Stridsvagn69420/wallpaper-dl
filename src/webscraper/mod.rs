use crate::extractors::{SelectAttr, src_tag_attr};
use url::{Url, ParseError};
use reqwest::Error;
use scraper::{Html, error::SelectorErrorKind};

mod alphacoders;
pub use alphacoders::WallAbyss;

mod artstation;

/// Webscraper Downloader Trait
/// 
/// Trait used for downloaders that rely on [Web scraping](https://en.wikipedia.org/wiki/Web_scraping).
/// Often the service in question does not have a (suitable) API or its API paywalled.
pub trait Webscraper {
	/// New Webscraper
	/// 
	/// Creates a new Webscraper.
	/// Requires the HTML source code as well as the requested URL.
	fn new(html: &str, url: &Url) -> ScraperResult<Self> where Self: Sized;

	/// Pointer to parsed HTML
	fn source_html(&self) -> &Html;

	/// Pointer to download selector
	fn selector_download(&self) -> &SelectAttr;

	/// Pointer to title selector
	fn selector_title(&self) -> &SelectAttr;

	/// Image ID
	/// 
	/// Usually extracted from the URL.
	fn image_id(&self) -> &str;

	/// Image URL
	/// 
	/// The source image URL that points to the actual high quality image.
	fn image_url(&self) -> ScraperResult<Url> {
		let Some(url) = src_tag_attr(self.source_html(), self.selector_download()) else {
			return Err(ScraperError::ParseError("HTML element or attribute not found.".to_string()));
		};
		Ok(Url::parse(url)?)
	}

	/// Image Title
	fn image_title(&self) -> ScraperResult<String> {
		let Some(title) = src_tag_attr(self.source_html(), self.selector_title()) else {
			return Err(ScraperError::ParseError("HTML element or attribute not found.".to_string()));
		};
		Ok(title.to_string())
	}
}

/// Scraper Result alias
/// 
/// Just an alias like [io::Result](std::io::Result).  
/// `T`: Any type provided  
/// `E`: [ScraperError]
pub type ScraperResult<T> = Result<T, ScraperError>;

/// Webscraper Errors
/// 
/// Simplified errors for [Webscraper] structs.
pub enum ScraperError {
	/// Connection Failed
	/// 
	/// The request could not be sent.
	/// This is most likely a core network issue.
	ConnectionFailed(String),

	/// Request Failed
	/// 
	/// The request was sent but the result was unexpected.
	RequestFailed(String),

	/// Parse Error
	/// 
	/// The website contents could not be parsed.
	ParseError(String),

	/// Other
	/// 
	/// More or less a dummy error.
	/// Used to map an [Option] to this type.
	Other
}

impl From<SelectorErrorKind<'_>> for ScraperError {
	fn from(value: SelectorErrorKind) -> Self {
		ScraperError::ParseError(value.to_string())
	}
}

impl From<Error> for ScraperError {
	fn from(value: Error) -> Self {
		let err = value.to_string();
		if value.is_builder() || value.is_connect() || value.is_redirect() {
			ScraperError::ConnectionFailed(value.to_string())
		} else if value.is_body() || value.is_decode() {
			ScraperError::ParseError(err)
		} else if value.is_request() || value.is_status() || value.is_timeout() {
			ScraperError::RequestFailed(err)
		} else {
			ScraperError::Other
		}
	}
}

impl From<ParseError> for ScraperError {
	fn from(value: ParseError) -> Self {
		ScraperError::ParseError(value.to_string())
	}
}