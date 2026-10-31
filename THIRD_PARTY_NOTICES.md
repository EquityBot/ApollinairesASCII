# Third-party notices

## RASCII

Source: https://github.com/orhnk/RASCII, commit `8d33a3670e51298d530ab202090b212501310cdd`.

Copyright (c) 2022-present KoBruh. MIT. The original notice remains in [LICENSE.md](LICENSE.md). AsciiForge retains upstream Git history and adapts its image-to-character conversion approach. Other components are new code.

## figlet-rs and font data

`figlet-rs` 1.0.0: https://github.com/yuanbohan/rs-figlet. Apache-2.0. The dependency supplies the FIGlet renderer and its bundled standard, small, big and slant FLF files. The Apache licence text and contributor notice are retained in [LICENSES/Apache-2.0.txt](LICENSES/Apache-2.0.txt) and [LICENSES/figlet-rs-COPYRIGHT.txt](LICENSES/figlet-rs-COPYRIGHT.txt).

Bundled font headers attribute Standard to Glenn Chappell and Ian Chai (March 1993), based on Frank's signature; Small to Glenn Chappell (April 1993); Big to Glenn Chappell (April 1993), with Greek characters by Bruce Jakeway; and Slant to Glenn Chappell (March 1993). Headers also credit modifications by John Cowan and Paul Burton where present. The font files remain unmodified inside the dependency. Their headers permit modification when the modifier's name is added to a comment line. AsciiForge's block and outline styles transform rendered output rather than editing font files.

## Other dependencies

Cargo.toml and Cargo.lock record direct and transitive Rust dependencies. The [browser notice bundle](web/THIRD_PARTY_NOTICES.txt) includes the root licence files for all 72 locked dependencies and the bundled font headers, covering native and browser builds. Refresh it when Cargo.lock changes. Dependencies retain their own licences; the repository's MIT licence does not replace those terms. The GIF dependency is pinned to an official upstream fix whilst awaiting a stable release with the newer LZW decoder; see [dependency notes](docs/DEPENDENCIES.md).

Ghostty, other compared projects and their artwork are not bundled. Original static templates and procedural scenes in this repository are covered by its MIT licence.
