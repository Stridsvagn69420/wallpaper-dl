# wallpaper-dl
[![Stars](https://img.shields.io/github/stars/Stridsvagn69420/wallpaper-dl.svg)][github]
[![Downloads](https://img.shields.io/crates/d/wallpaper-dl.svg)][crate]
[![Version](https://img.shields.io/crates/v/wallpaper-dl.svg)][crate]
[![License](https://img.shields.io/crates/l/wallpaper-dl.svg)][crate]

[crate]: https://crates.io/crates/wallpaper-dl
[github]: https://github.com/Stridsvagn69420/wallpaper-dl

Web scraper for downloading wallpapers from various sites

## Supported Websites
Currently it's very work-in-progress, but I plan on supporting these websites:
- [Wallhaven](https://wallhaven.cc/)
- Alphacoders ([Wallpaper Abyss](https://wall.alphacoders.com/), [Art Abyss](https://art.alphacoders.com/), [Image Abyss](https://pics.alphacoders.com/))
- [ArtStation](https://www.artstation.com/)

### Potential canidates
- [DeviantArt](https://www.deviantart.com/) (see note below)
- [Twitter / X](https://twitter.com)
- [Bluesky](https://bsky.app/)
- [Mastodon](https://mastodon.social/) (see note below)

I only put it DeviantArt here due to its size, but only as a canidate, because it frankly sucks. Most images are now AI generated (yet low in resolution) or not even officially downloadable in full resolution. For the latter, I found [this](https://www.deviantart.com/ellysiumn/art/City-of-fog-853408315) to be the worst case so far. It *is* an 8K image, but the only resolution you can get without paying is __600 x 300__! I might add it in the future, but it does not seem worth it to me.

As for Mastodon, I think this one is worth it. The only issue probably is to check if the URL is from a Mastodon instance, but there's most likely some API to check that based on the Host parameter in the URL.

## Configuration
This is a mock-up configuration with default values set (used if config is missing):
```toml
[storage]
basedir = "~/Pictures/Wallpapers" # Root directory of your wallpaper collection. Resolves "~", maybe also env vars.
sort = "website" # How to sort using subdirectories. "website", "domain", "artist", "none". Directory names will be treated case-insensitive. Can be overwritted per execution via flags.
format = "{width}x{height}_{title}_{id}" # How filenames should be formatted (excluding the extension). {original} will use the filename as if you were to download it via a browser. WIP!!! But will attempt to find a file name by hints, e.g. URL or Content-Dispositon.

[download]
delay = 300 # Per-website artificial delay in milliseconds to mitigate timeouts.
```
This will most likely change during development.