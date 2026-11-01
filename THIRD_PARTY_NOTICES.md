# Third-party notices

## RASCII

Source: https://github.com/orhnk/RASCII, commit `8d33a3670e51298d530ab202090b212501310cdd`.

Copyright (c) 2022-present KoBruh. MIT. The original notice remains in [LICENSE.md](LICENSE.md). ApollinairesASCII retains upstream Git history and adapts its image-to-character conversion approach. Other components are new code.

## FIGlet renderer and fonts

Source: https://github.com/cmatsuoka/figlet, commit `202a0a8110650a943f1125f536b3bb455cf72ee1` (FIGlet 2.2.5). BSD-3-Clause; Copyright (C) 1991, 1993, 1994 Glenn Chappell and Ian Chai, 1996-2001 John Cowan, 2002 Christiaan Keet, 2011 Claudio Matsuoka. The licence is retained in [LICENSES/FIGlet-BSD-3-Clause.txt](LICENSES/FIGlet-BSD-3-Clause.txt). Debian's copyright record for figlet applies this licence to every file in the distribution, including the fonts.

`src/lettering.rs` ports FIGlet's horizontal layout, smushing rules and word wrap to Rust. `src/fonts/` bundles 15 of the 18 distribution fonts: banner, big, block (named `figlet-block`, because `block` is the existing filled style), ivrit, lean, mini, mnemonic, script, shadow, slant, small, smscript, smshadow, smslant and standard. Each is trimmed to its header, comments and the 102 required FIGcharacters; an added comment line records the trim, as the font headers require of modifiers. Bubble, digital and term are not bundled: their required German FIGcharacters contain byte 0xC4, outside printable ASCII. Each font keeps its header crediting its authors, and the [browser notice bundle](web/THIRD_PARTY_NOTICES.txt) reproduces every bundled header.

## figlet.js vertical layout

Source: https://github.com/patorjk/figlet.js, version 1.12.0. MIT; Copyright (C) 2014-present Patrick Gillespie and other contributors. The licence is retained in [LICENSES/figlet.js-MIT.txt](LICENSES/figlet.js-MIT.txt). `src/lettering.rs` ports its vertical fitting and smushing to Rust. Unlike figlet.js, a font's vertical rules apply only when its header or the user selects vertical smushing, as the FIGfont 2 specification states.

## TOIlet fonts

Source: https://github.com/cacalabs/toilet, commit `3eb9d58037afb0a1baec6dca82caa045fb2217c0`. WTFPL-2; Copyright (C) 2004 Sam Hocevar. The licence is retained in [LICENSES/TOIlet-WTFPL-2.txt](LICENSES/TOIlet-WTFPL-2.txt). `src/fonts/` bundles ascii9, ascii12, bigascii9, bigascii12, smascii9 and smascii12, extracted from their ZIP containers and trimmed like the FIGlet fonts. The other 19 distribution fonts are not bundled: letter's required German FIGcharacters contain non-ASCII bytes, and the rest draw printable ASCII characters with Unicode block, Braille, box-drawing or Cyrillic characters.

figlet.js 1.12.0 ships 332 fonts. Its fonts are not bundled: 20 of the 21 whose headers state a redistribution licence draw with non-ASCII characters, the remaining one (Font Font, MIT) is not widely used, and the others state no redistribution licence.

## Other dependencies

Cargo.toml and Cargo.lock record direct and transitive Rust dependencies. The [browser notice bundle](web/THIRD_PARTY_NOTICES.txt) includes the root licence files for all 102 locked dependencies, the FIGlet, figlet.js and TOIlet licences and the bundled font headers, covering native and browser builds. Release archives include it as `DEPENDENCY_NOTICES.txt`. Refresh it when Cargo.lock changes. The CLI's HTTPS client (`ureq`, `rustls`, `ring` and `webpki-roots`) adds ISC, BSD-3-Clause and CDLA-Permissive-2.0 terms alongside MIT and Apache-2.0; these crates are not part of the library without the `cli` feature or of the browser build. Dependencies retain their own licences; the repository's MIT licence does not replace those terms. The GIF dependency is pinned to an official upstream fix whilst awaiting a stable release with the newer LZW decoder; see [dependency notes](docs/DEPENDENCIES.md).

Ghostty, other compared projects and their artwork are not bundled. The 89 catalogue pieces in `src/art.rs`, the text effects and the procedural scenes were made for this repository and are covered by its MIT licence. No catalogue artwork was copied from another project.
