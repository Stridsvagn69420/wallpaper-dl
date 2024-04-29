# wallpaper-dl
[![Stars](https://img.shields.io/github/stars/Stridsvagn69420/wallpaper-dl.svg)][github]
[![Downloads](https://img.shields.io/crates/d/wallpaper-dl.svg)][crate]
[![Version](https://img.shields.io/crates/v/wallpaper-dl.svg)][crate]
[![License](https://img.shields.io/crates/l/wallpaper-dl.svg)][crate]

[crate]: https://crates.io/crates/wallpaper-dl
[github]: https://github.com/Stridsvagn69420/wallpaper-dl

Wallpaper downloader for various websites

## Supported Websites
- [Wallhaven](https://wallhaven.cc/)
- Alphacoders ([Wallpaper Abyss](https://wall.alphacoders.com/), [Art Abyss](https://art.alphacoders.com/), [Image Abyss](https://pics.alphacoders.com/))
- [ArtStation](https://www.artstation.com/)

## Usage
Downloading wallpapers:
```bash
wallpaper-dl <URLs>
```

Setting the current wallpaper:
```bash
wallpaper-dl current <URL/Path>
```

Getting the current wallpaper path:
```bash
wallpaper-dl current
```

## Contributing
Want to add a feature, enhance website support or report a bug? Just open a new issue on [GitHub][github]!

#### Ideas for version 0.2.0
It's a hobby project of mine, so I can't just work on it 24/7, but here are some things that I want to implement:
- Removing missing files from database
- Tokio multithreaded and async/await
- Buffered IO for downloading (requires async due to streams)
- [indicatif](https://github.com/console-rs/indicatif) for cleaner status output