use crate::config::{Config, Sort};
use crate::downloaders::{quick_get, DownloaderResult, WallpaperMeta};
use apputils::{paint, paintln, Colors};
use blake3::hash;
use mailparse::parse_content_disposition;
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
	/// Returns the absolute path of the file.
	///
	/// Currently, it does it in an unbuffered manner due to [download_file](Filedown::download_file)'s restrictions.
	pub fn save_file(
		dir: PathBuf,
		file: impl AsRef<Path>,
		data: &[u8],
	) -> io::Result<PathBuf> {
		let abspath = dir.join(file);
		fs::create_dir_all(&dir)?;

		let mut file = fs::File::create(&abspath)?;
		file.write_all(data)?;
		Ok(abspath)
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
		delay: u64,
		wallmeta: &WallpaperMeta,
		wallfile: Url,
	) -> DownloaderResult<(Vec<u8>, String, String)> {
		// Download file
		let mut resp = quick_get(client, wallfile, delay)?;

		// Filename canidates
		let title = wallmeta.title.as_ref();
		let head = resp.headers().to_owned();
		let path = resp.url().path().to_owned();

		// Download file into RAM
		let mut data = Vec::new();
		resp.copy_to(&mut data)?;

		// Parse Name
		let name = Filedown::file_name(title, &head, &path).unwrap_or_else(|| hash(&data).to_string());
		let ext = Filedown::file_ext(&head).unwrap_or("bin");
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
	/// Automatically creates the *relative* path with the given sorting method.
	/// This still needs to be joined with the root.
	pub fn resolve_path(conf: &Config, host: &str, tags: &[String]) -> PathBuf {
		let parent = Filedown::most_matching_genre(conf, tags).unwrap_or_else(|| host.to_string());
		Path::new(&parent).to_path_buf()
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
		let mut bestmatch = Filedown::match_genre(&first.1, walltags);
	
		// Find most suitable tag
		for (genre, tags) in genre_iter {
			let matchness = Filedown::match_genre(&tags, walltags);
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
	fn file_ext(heads: &HeaderMap) -> Option<&str> {
		let head = heads.get(CONTENT_TYPE)?;
		let ext = match head.to_str().ok()? {
			"image/bmp" => "bmp",
			"image/apng" => "apng",
			"image/png" => "png",
			"image/jpeg" => "jpg",
			"image/gif" => "gif",
			"image/avif" => "avif",
			"image/webp" => "webp",
			_ => "bin"
		};
		Some(ext)
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
			.or_else(|| Filedown::filename_content_disposition(condisp.get(CONTENT_DISPOSITION)))
			.or_else(|| Filedown::filename_url_path(path))
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

	pub fn cfg_wallpaper() -> ExitCode {
		paintln!(Colors::Yellow, "No wallpaper was set!");
		ExitCode::FAILURE
	}

	pub fn wall_set() -> ExitCode {
		paintln!(Colors::Green, "Wallpaper successfully updated");
		ExitCode::SUCCESS
	}	
}

/// Download errors
/// 
/// Error messages for the main download phase.
pub struct DownErr;
impl DownErr {
	/// Exitcode evaluator
	/// 
	/// The **first** parameter should be true for **database** write failures.  
	/// The **second** parameter should be true for **config** write failures.
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