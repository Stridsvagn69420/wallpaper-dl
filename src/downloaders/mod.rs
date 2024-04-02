//! Image Downloaders
//!
//! The submodule containing the wallpaper downloader and its helper functions.

use async_trait::async_trait;
use reqwest::{Client, Error, IntoUrl, Response};
use scraper::error::SelectorErrorKind;
use scraper::{Html, Selector};
use std::fmt;
use url::{ParseError, Url};

mod alphacoders;
mod artstation;
mod wallhaven;

pub use alphacoders::{ArtAbyss, ImageAbyss, WallpaperAbyss};
pub use artstation::ArtStation;
pub use wallhaven::Wallhaven;

const WALLHAVEN: &str = "wallhaven.cc";
const WALLPAPER_ABYSS: &str = "wall.alphacoders.com";
const ART_ABYSS: &str = "art.alphacoders.com";
const IMAGE_ABYSS: &str = "pics.alphacoders.com";
const ARTSTATION: &str = "artstation.com";

macro_rules! ok_box {
	($dl:expr) => {
		Ok(Box::new($dl.await?))
	};
}

/// From URL
///
/// Returns the [Downloader] needed for the provided [Url].
pub async fn from_url(c: &Client, url: Url) -> DownloaderResult<Box<dyn Downloader>> {
	let host = url.host_str().unwrap_or_default();

	if host.ends_with(WALLHAVEN) {
		ok_box!(Wallhaven::new(c, url))
	} else if host.ends_with(WALLPAPER_ABYSS) {
		ok_box!(WallpaperAbyss::new(c, url))
	} else if host.ends_with(ART_ABYSS) {
		ok_box!(ArtAbyss::new(c, url))
	} else if host.ends_with(IMAGE_ABYSS) {
		ok_box!(ImageAbyss::new(c, url))
	} else if host.ends_with(ARTSTATION) {
		ok_box!(ArtStation::new(c, url))
	} else {
		Err(DownloaderError::Other)
	}
}

/// Webscraper Downloader Trait
///
/// Trait for Wallpaper downloaders that rely on webscraping.
#[async_trait]
pub trait Downloader {
	/// New Wallpaper Downloader
	///
	/// Creates a new Webscraper.
	/// Requires a preconfigured [Client] as well as the post [Url].
	async fn new(client: &Client, url: Url) -> DownloaderResult<impl Downloader>
	where
		Self: Sized;

	/// Image ID
	///
	/// Usually extracted from the URL. Has got to always exist logically.
	/// If the ID is not suitable to look like an ID, something like a hash is also suitable.
	fn image_id(&self) -> &str;

	/// Image URL
	///
	/// The source image URL that points to the actual high quality image.
	/// The main thread will handle the file downloading.
	fn image_url(&self) -> DownloaderResult<Urls>;

	/// Image Title
	///
	/// The title of the image. Can also be a mix of the title and artist.
	/// If a website does not use titles for some odd reason, the downloader should return the errror [Other](DownloaderError::Other)
	/// in order to let the main thread know that it should look for title hints when downloading the data
	/// or hash it and use that as a title.
	fn image_title(&self) -> DownloaderResult<String>;

	/// Image Tags
	///
	/// Returns a collection of tags related to the image.
	/// If a website does not use tags, it should attempt to parse useful data out of other elements, e.g. the title or URL.
	/// A [Other error](DownloaderError::Other) means it could not find any tags.
	///
	/// The Downloaders themselves don't have to remove duplicates,
	/// but the idea is that the main thread will sort and [dedup](Vec::dedup) anyway,
	/// so that it does not have to be implemented every single time.
	fn image_tags(&self) -> DownloaderResult<Vec<String>>;
}

/// URL Quantity
///
/// Since some websites have multiple images per post,
/// this enum carries either a single [Url] or a [Vec] of it,
/// so that the main thread does not have to deal with uneccesary Array operations.
pub enum Urls {
	/// A single URL
	Single(Url),

	/// Multiple URLs
	Multi(Vec<Url>),
}

impl From<Url> for Urls {
	fn from(value: Url) -> Self {
		Urls::Single(value)
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
/// Simplified errors for [Downloader] structs.
#[derive(Debug)]
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
	Other,
}

impl fmt::Display for DownloaderError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let msg = match self {
			DownloaderError::ConnectionFailed(x) => x,
			DownloaderError::RequestFailed(x) => x,
			DownloaderError::ParseError(x) => x,
			DownloaderError::Other => "Could not retrieve any value",
		};
		write!(f, "{msg}")
	}
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

/// Quick download wrapper
///
/// Shorthand for sending a GET request. If successful, the [Response]'s body can be used.
async fn quick_get(c: &Client, url: impl IntoUrl) -> DownloaderResult<Response> {
	let resp = c.get(url).send().await?.error_for_status()?;
	Ok(resp)
}

/// Selector and Attribute
///
/// A combination of a [Selector] and an attribute.
/// Useful for Webscraping-based downloaders.
pub struct SelectAttr {
	pub select: Selector,
	pub attr: String,
}

impl SelectAttr {
	/// Parse to Selector and Attribute
	/// 
	/// Parses the CSS-Selector and combines it with the attribute name into a [SelectAttr].
	pub fn parse<'a>(css: &'a str, attr: &'a str) -> Result<Self, SelectorErrorKind<'a>> {
		Ok(Self {
			select: Selector::parse(css)?,
			attr: attr.to_string(),
		})
	}

	/// Extract from HTML
	/// 
	/// A simple built-in attribute extractor.
	/// It tries to find the element in the provided [Html],
	/// then reads the attribute and returns it as a &[str].
	pub fn extract<'a>(&self, html: &'a Html) -> DownloaderResult<&'a str> {
		match html
			.select(&self.select)
			.next()
			.and_then(|x| x.attr(&self.attr))
		{
			None => Err(DownloaderError::ParseError("HTML element or attribute not found.".to_string())),
			Some(x) => Ok(x)
		}
	}
}

/// Webscraper extractor wrappers
///
/// Basically just a collection of functions that are re-used by webscraper downloaders.
struct ScraperWrapper;
impl ScraperWrapper {
	/// Image URL wrapper
	///
	/// Extract value from element and parse it as a [Url].
	pub fn image_url(html: &Html, select: &SelectAttr) -> DownloaderResult<Url> {
		let link = select.extract(html)?;
		Ok(Url::parse(link)?)
	}

	/// Image Title wrapper
	///
	/// Extract value from element and convert it to a [String].
	pub fn image_title(html: &Html, select: &SelectAttr) -> DownloaderResult<String> {
		let title = select.extract(html)?;
		Ok(title.to_string())
	}
}