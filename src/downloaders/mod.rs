//! Image Downloaders
//!
//! The submodule containing the wallpaper downloader and its helper functions.

use apputils::{paint, Colors};
use reqwest::blocking::{Client, Response};
use reqwest::{Error, IntoUrl};
use scraper::error::SelectorErrorKind;
use scraper::{Html, Selector};
use std::fmt;
use std::thread;
use std::time::Duration;
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
		Ok(Box::new($dl?))
	};
}

/// [Downloader] from URL
///
/// Returns the [Downloader] needed for the provided [Url].
pub fn from_url(c: &Client, url: Url, d: u64) -> DownloaderResult<Box<dyn Downloader>> {
	let host = url.host_str().unwrap_or_default();

	paint!(Colors::GreenBold, "  Fetching ");
	print!("{url}");

	if host.ends_with(WALLHAVEN) {
		ok_box!(Wallhaven::new(c, url, d))
	} else if host.ends_with(WALLPAPER_ABYSS) {
		ok_box!(WallpaperAbyss::new(c, url, d))
	} else if host.ends_with(ART_ABYSS) {
		ok_box!(ArtAbyss::new(c, url, d))
	} else if host.ends_with(IMAGE_ABYSS) {
		ok_box!(ImageAbyss::new(c, url, d))
	} else if host.ends_with(ARTSTATION) {
		ok_box!(ArtStation::new(c, url, d))
	} else {
		Err(DownloaderError::Other)
	}
}

/// Webscraper Downloader Trait
///
/// Trait for Wallpaper downloaders that rely on webscraping.
pub trait Downloader {
	/// New Wallpaper Downloader
	///
	/// Creates a new Webscraper.
	/// Requires a preconfigured [Client] as well as the post [Url].
	fn new(client: &Client, url: Url, delay: u64) -> DownloaderResult<impl Downloader>
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
}

/// Wallpaper Metadata
/// 
/// Bundled output of a [Downloader]
pub struct WallpaperMeta {
	/// Post ID
	/// 
	/// A unique identifier for the Wallpaper/Post.
	pub id: String,

	/// Wallpaper Title
	/// 
	/// The title of the Wallpaper, if it exists.
	pub title: Option<String>,

	/// Wallpaper URL
	/// 
	/// The URL (or multiple URLs) of a Wallpaper.
	pub images: Urls
}

impl TryFrom<Box<dyn Downloader>> for WallpaperMeta {
	type Error = DownloaderError;

	fn try_from(value: Box<dyn Downloader>) -> Result<Self, Self::Error> {
		// Extract title
		let title = match value.image_title() {
			Ok(x) => Some(x),
			Err(err) => match err {
				DownloaderError::Other => None,
				_ => return Err(err)
			}
		};

		// Bundle information
		let meta = Self {
			title,
			id: value.image_id().to_owned(),
			images: value.image_url()?
		};
		Ok(meta)
	}
}

/// URL Quantity
///
/// Since some websites have multiple images per post,
/// this enum carries either a single [Url] or a [Vec] of it,
/// so that the main thread does not have to deal with uneccesary Array operations.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Urls {
	/// A single URL
	Single(Url),

	/// Multiple URLs
	Multi(Vec<Url>),
}

impl Urls {
	/// Is [Single](Urls::Single)?
	pub fn is_single(&self) -> bool {
		match self {
			Urls::Single(_) => true,
			Urls::Multi(x) => x.len() < 2,
		}
	}
}

impl From<Url> for Urls {
	fn from(value: Url) -> Self {
		Urls::Single(value)
	}
}

impl From<Urls> for Vec<Url> {
	fn from(value: Urls) -> Self {
		match value {
			Urls::Single(x) => vec![x],
			Urls::Multi(y) => y,
		}
	}
}

impl From<&Urls> for Vec<Url> {
	fn from(value: &Urls) -> Self {
		match value {
			Urls::Single(x) => vec![x.clone()],
			Urls::Multi(y) => y.to_vec(),
		}
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
	Other
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
pub fn quick_get(c: &Client, url: impl IntoUrl, delay: u64) -> DownloaderResult<Response> {
	let resp = c.get(url).send()?.error_for_status()?;
	thread::sleep(Duration::from_millis(delay));
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