# Contributing

## The contract

Artwork stays printable ASCII: bytes 32 to 126, rows separated by line feeds, shown in the browser as DOM text. Do not add Unicode, Braille, colour, canvas, SVG or raster artwork output. Image and video files are fine as inputs.

## Reuse

Code, fonts and artwork from other projects need a licence that permits redistribution under MIT. Record their source and licence in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) and, for anything bundled into the browser editor, `web/THIRD_PARTY_NOTICES.txt`. Original art is welcome.

## Checks

Run these before opening a pull request:

```bash
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo clippy --locked --target wasm32-unknown-unknown --no-default-features --features web -- -D warnings
```

New features and bug fixes come with tests in `tests/` or a `#[cfg(test)]` module. For editor changes, build the browser module as the README describes, serve `web/` and check the change by hand, including keyboard use. CI runs the native checks on Linux and Windows and builds the editor.
