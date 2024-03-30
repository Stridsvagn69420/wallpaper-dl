use url::{Url, ParseError};
use reqwest::{Client, Error, Response};
use scraper::error::SelectorErrorKind;

mod alphacoders;
pub use alphacoders::{WallAbyss, ArtAbyss, ImageAbyss};

mod artstation;
pub use artstation::ArtStation;

/// Quick download wrapper
/// 
/// Shorthand for sending a get request. If successful, the [Response]'s body can be used.
async fn quick_get(c: &Client, url: Url) -> DownloaderResult<Response> {
	let resp = c.get(url)
			.send().await?
			.error_for_status()?;
	Ok(resp)
}

/// Webscraper Downloader Trait
/// 
/// Trait for Wallpaper downloaders that rely on webscraping.
pub trait ImageDownloader {
	/// New Wallpaper Downloader
	/// 
	/// Creates a new Webscraper.
	/// Requires a preconfigured [Client] as well as the post [Url].
	async fn new(client: &Client, url: Url) -> DownloaderResult<Self> where Self: Sized;

	/// Image ID
	/// 
	/// Usually extracted from the URL.
	fn image_id(&self) -> &str;

	/// Image URL
	/// 
	/// The source image URL that points to the actual high quality image.
	async fn image_url(&self) -> DownloaderResult<Url>;

	/// Image Title
	/// 
	/// The title of the image. Can also be a mix of the title and artist.
	async fn image_title(&self) -> DownloaderResult<String>;
}

/// Downloader Result alias
/// 
/// Just an alias like [io::Result](std::io::Result).  
/// `T`: Any type provided  
/// `E`: [DownloaderError]
pub type DownloaderResult<T> = Result<T, DownloaderError>;

/// Downloader Errors
/// 
/// Simplified errors for [ImageDownloader] structs.
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