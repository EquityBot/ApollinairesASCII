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
cargo clippy --locked --target wasm32-unknown-unknown --no-default-features --features web --lib --tests -- -D warnings
cargo test-editor
```

New features and bug fixes come with tests in `tests/` or a `#[cfg(test)]` module. Editor changes come with a case in `src/web/tests.rs`, driven by pointer and keyboard as a user would; `cargo test-editor` runs them, and the README lists what they need. Check layout and anything the suite cannot reach, such as the system clipboard, by hand in a served `web/`. CI runs the native checks on Linux and Windows, runs the editor tests in headless Chrome and builds the editor.
