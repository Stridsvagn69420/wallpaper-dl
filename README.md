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
- (BROKEN) Alphacoders ([Wallpaper Abyss](https://wall.alphacoders.com/), [Art Abyss](https://art.alphacoders.com/), [Image Abyss](https://pics.alphacoders.com/))
- [ArtStation](https://www.artstation.com/)

Please open a new issue on [GitHub][github], if you want to have a website added or fixed (i.e. the current implementation is broken).

### To-Do
#### This version
- add help messages
- manual adding of wallpapers into the database (mitigates broken websites)
- manual removing of wallpapers <!--similar code to subcommand current-->

#### Next version
- Tokio multithreaded and async/await
- Buffered IO for downloading (requires async due to streams)
- [indicatif](https://github.com/console-rs/indicatif) for cleaner status output