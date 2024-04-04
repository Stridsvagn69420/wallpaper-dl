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

### Potential canidates
- [DeviantArt](https://www.deviantart.com/) (see note below)
- [Twitter / X](https://twitter.com)
- [Bluesky](https://bsky.app/)
- [Mastodon](https://mastodon.social/) (see note below)

I put it DeviantArt here due to its size, but only as a canidate, because it frankly sucks. Most images are now AI generated (yet low in resolution) or not even officially downloadable in full resolution. For the latter, I found [this](https://www.deviantart.com/ellysiumn/art/City-of-fog-853408315) to be the worst case so far. It *is* an 8K image, but the only resolution you can get without paying is __600 x 300__! I might add it in the future, but it does not seem worth it to me.

As for Mastodon, I think this one is worth it. The only issue probably is to check if the URL is from a Mastodon instance, but there's most likely some API to check that based on the Host parameter in the URL.