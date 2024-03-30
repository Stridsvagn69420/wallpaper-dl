use crate::extractors::{SelectAttr, src_tag_attr};
use url::{Url, ParseError};
use reqwest::Error;
use scraper::{Html, error::SelectorErrorKind};

mod alphacoders;
pub use alphacoders::{WallAbyss, ArtAbyss, ImageAbyss};

mod artstation;
pub use artstation::ArtStation;

/// Webscraper Downloader Trait
/// 
/// Trait for Wallpaper downloaders that rely on webscraping.
pub trait Webscraper {
	/// New Wallpaper Downloader
	/// 
	/// Creates a new Webscraper.
	/// Requires the HTML source code as well as the requested URL.
	fn new(html: &str, url: &Url) -> DownloaderResult<Self> where Self: Sized;

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
	fn image_url(&self) -> DownloaderResult<Url> {
		let Some(url) = src_tag_attr(self.source_html(), self.selector_download()) else {
			return Err(DownloaderError::ParseError("HTML element or attribute not found.".to_string()));
		};
		Ok(Url::parse(url)?)
	}

	/// Image Title
	fn image_title(&self) -> DownloaderResult<String> {
		let Some(title) = src_tag_attr(self.source_html(), self.selector_title()) else {
			return Err(DownloaderError::ParseError("HTML element or attribute not found.".to_string()));
		};
		Ok(title.to_string())
	}
}

/// Downloader Result alias
/// 
/// Just an alias like [io::Result](std::io::Result).  
/// `T`: Any type provided  
/// `E`: [DownloaderError]
pub type DownloaderResult<T> = Result<T, DownloaderError>;

/// Downloader Errors
/// 
/// Simplified errors for [Webscraper] and [Restapi] structs.
pub enum DownloaderError {
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
	/// Related website data could not be parsed.
	ParseError(String),

	/// Other
	/// 
	/// More or less a dummy error.
	/// Used to map an [Option] to this type.
	Other
}

impl From<SelectorErrorKind<'_>> for DownloaderError {
	fn from(value: SelectorErrorKind) -> Self {
		DownloaderError::ParseError(value.to_string())
	}
}

impl From<Error> for DownloaderError {
	fn from(value: Error) -> Self {
		let err = value.to_string();
		if value.is_builder() || value.is_connect() || value.is_redirect() {
			DownloaderError::ConnectionFailed(value.to_string())
		} else if value.is_body() || value.is_decode() {
			DownloaderError::ParseError(err)
		} else if value.is_request() || value.is_status() || value.is_timeout() {
			DownloaderError::RequestFailed(err)
		} else {
			DownloaderError::Other
		}
	}
}

impl From<ParseError> for DownloaderError {
	fn from(value: ParseError) -> Self {
		DownloaderError::ParseError(value.to_string())
	}
}