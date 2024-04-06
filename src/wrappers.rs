use crate::config::{Config, Sort};
use crate::downloaders::{quick_get, DownloaderError, DownloaderResult, WallpaperMeta};
use apputils::{paint, paintln, Colors};
use blake3::hash;
use mailparse::parse_content_disposition;
use mime_guess::get_mime_extensions_str;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_DISPOSITION, CONTENT_TYPE};
use std::fmt::Display;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use url::Url;

/// File Download Wrappers
///
/// Wrappers around downloading and saving a wallpaper.
pub struct Filedown;
impl Filedown {
	/// Save Wallpaper file
	///
	/// Wrapper around saving the downloaded wallpaper to disk.
	///
	/// Currently, it does it in an unbuffered manner due to [download_file](Filedown::download_file)'s restrictions.
	pub fn save_file(
		dir: impl AsRef<Path>,
		file: impl AsRef<Path>,
		data: &[u8],
	) -> io::Result<()> {
		fs::create_dir_all(dir)?;
		let mut file = fs::File::create(file)?;
		file.write_all(data)
	}

	/// Download Wallpaper file
	///
	/// Wrapper around downloading a file into RAM and getting a usable parameter for `{title}` and `{ext}` when formatting.
	///
	/// Currently, it does it in an unbuffered manner, because the blocking version of [reqwest](reqwest::blocking) does not have a byte stream,
	/// hence why it needs to be cached in RAM.  
	/// A future release of **wallpaper-dl** will use the asynchronous API to increase performance.
	pub fn download_file(
		client: &Client,
		wallmeta: &WallpaperMeta,
		wallfile: Url,
	) -> DownloaderResult<(Vec<u8>, String, String)> {
		// Download file
		let mut resp = quick_get(client, wallfile)?;

		// Filename canidates
		let title = wallmeta.title.as_ref();
		let head = resp.headers().to_owned();
		let path = resp.url().path().to_owned();

		// Download file into RAM
		let mut data = Vec::new();
		resp.copy_to(&mut data)?;

		// Parse Name
		let name = file_name(title, &head, &path).unwrap_or_else(|| hash(&data).to_string());
		let ext = file_ext(&head).unwrap_or(&"bin");
		Ok((data, name, ext.to_string()))
	}

	/// Format filename
	/// 
	/// Automatically formats the filename, including for those that are one of many images of a post.
	pub fn format_file(meta: &WallpaperMeta, name: impl Display, ext: impl Display, count: Option<u8>) -> String {
		if let Some(c) = count {
			format!("{}-{name}-{c}.{ext}", meta.id)
		} else {
			format!("{}-{name}.{ext}", meta.id)
		}
	}

	/// Resolve parent path
	/// 
	/// Automatically creates the path with the given sorting method.
	pub fn resolve_path(conf: &Config, host: &str, tags: &[String]) -> PathBuf {
		let parent = most_matching_genre(conf, tags).unwrap_or_else(|| host.to_string());
		conf.download.path.join(parent)
	}
}

fn most_matching_genre(conf: &Config, walltags: &[String]) -> Option<String> {
	// Iteratre over configured genres
	if conf.download.sort != Sort::Genres {
		return None;
	}
	let mut genre_iter = conf.genres.clone()?.into_iter();

	// Initial values
	let first = genre_iter.next()?;
	let mut bestgenre = first.0;
	let mut bestmatch = match_genre(&first.1, walltags);

	// Find most suitable tag
	for (genre, tags) in genre_iter {
		let matchness = match_genre(&tags, walltags);
		if matchness > bestmatch {
			bestgenre = genre;
			bestmatch = matchness;
		}
	}
	Some(bestgenre)
}

/// Match genre O(n^2)
/// 
/// Matches a genre-slice with an image tag-slice. Note that the `image` slice must be sorted!
fn match_genre(genre: &[String], image: &[String]) -> usize {
	genre.iter().filter(|x| image.binary_search(x).is_ok()).count()
}

/// Get File Extension
///
/// Checks the `Content-Type` header for the required file extension.
fn file_ext(heads: &HeaderMap) -> Option<&&str> {
	let head = heads.get(CONTENT_TYPE)?;
	let hv = get_mime_extensions_str(head.to_str().ok()?)?;
	hv.first()
}

/// Find suitable file name
///
/// Attempts to find a suitable file name in this order:
/// 1. The API-provided title
/// 2. The `filename` set in Content-Disposition
/// 3. The [file_stem](Path::file_stem) of the URL Path
///
/// A [None] means that absolutely no title could be found.
fn file_name(title: Option<&String>, condisp: &HeaderMap, path: &str) -> Option<String> {
	title
		.map(|x| x.to_owned())
		.or_else(|| filename_content_disposition(condisp.get(CONTENT_DISPOSITION)))
		.or_else(|| filename_url_path(path))
}

/// Get filename from Content-Disposition
fn filename_content_disposition(condisp: Option<&HeaderValue>) -> Option<String> {
	let header = condisp?.to_str().ok()?;
	parse_content_disposition(header)
		.params
		.get("filename")
		.cloned()
}

/// Get filename from URL-Path
fn filename_url_path(path: &str) -> Option<String> {
	let filename = Path::new(path).file_stem().and_then(|x| x.to_str())?;
	Some(filename.to_string())
}

/// Main Thread Errors
///
/// Messages for errors that occur on the main thread, excluding the download phase.
pub struct MainErr;
impl MainErr {
	pub fn db_load() -> ExitCode {
		paintln!(Colors::RedBold, "Failed to read wallpaper database!");
		ExitCode::FAILURE
	}

	pub fn db_load_notfound() -> ExitCode {
		paintln!(Colors::YellowBold, "Wallpaper database not found!");
		ExitCode::FAILURE
	}

	pub fn db_not_found(c: Colors, t: &str) -> ExitCode {
		paintln!(c, "{t} not found in database!");
		ExitCode::FAILURE
	}

	pub fn db_param_not_found(
		ctyp: Colors,
		cdata: Colors,
		typ: &str,
		payload: impl Display,
	) -> ExitCode {
		highlight(ctyp, cdata, typ, payload, " not found in database!")
	}

	pub fn cfg_load(err: io::Error) -> ExitCode {
		match err.kind() {
			io::ErrorKind::NotFound => paintln!(Colors::YellowBold, "Config not found!"),
			_ => return key_value( Colors::RedBold, Colors::Red, "Failed to read config file", err)
		}
		ExitCode::FAILURE
	}

	pub fn cfg_save(ctext: Colors, cerr: Colors, err: io::Error) -> ExitCode {
		key_value(ctext, cerr, "Failed to save configuration", err)
	}

	pub fn cfg_wallpaper() -> ExitCode {
		paintln!(Colors::Yellow, "No wallpaper was set!");
		ExitCode::FAILURE
	}
}

/// Download errors
/// 
/// Error messages for the main download phase.
pub struct DownErr;
impl DownErr {
	/// No (valid) URLs were found
	pub fn valid_urls() -> ExitCode {
		paintln!(Colors::RedBold, "No valid URLs provided!");
		ExitCode::FAILURE
	}

	/// Input already listed in database
	pub fn new_urls() -> ExitCode {
		paintln!(Colors::Cyan, "No new wallpapers to download!");
		ExitCode::SUCCESS
	}

	/// Initial request failed
	pub fn init_req<T>(err: DownloaderError, host: &str) -> Option<T> {
		match err {
			DownloaderError::Other => paintln!(Colors::YellowBold, "{host} is not supported!"),
			_ => paintln!(Colors::Red, "{err}"), // TODO: Improve
		}
		None
	}

	/// Could not create HTTP-Client
	pub fn tls_resolve() -> ExitCode {
		key_value(
			Colors::Red,
			Colors::RedBold,
			"Failed to initialize reqwest client",
			"DNS resolver settings could not be parsed",
		)
	}

	/// Exitcode evaluator
	pub fn finish(config: bool, database: bool) -> ExitCode {
		if database {
			paint!(Colors::RedBold, "CRITICAL! ");
			paintln!(Colors::Red, "Failed to save database!");
		}
		if config {
			paint!(Colors::RedBold, "CRITICAL! ");
			paintln!(Colors::Red, "Failed to save config file!");
		}
		if config || database {
			ExitCode::FAILURE
		} else {
			ExitCode::SUCCESS
		}
	}
}

/// Key-Value error formatting
///
/// Prints out an error in a Key-Value-like style.
fn key_value(ctext: Colors, cerr: Colors, text: &str, err: impl Display) -> ExitCode {
	paint!(ctext, "{text}: ");
	paintln!(cerr, "{err}");
	ExitCode::FAILURE
}

/// Highlighting error formatting
///
/// Highlights a specifc value with another color.
fn highlight(
	cn: Colors,
	ch: Colors,
	pre: impl Display,
	main: impl Display,
	post: impl Display,
) -> ExitCode {
	paint!(cn, "{pre} ");
	paint!(ch, "{main}");
	paintln!(cn, " {post}");
	ExitCode::FAILURE
}