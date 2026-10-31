# ApollinairesASCII

Static art, block letters and animated scenes made of real, selectable ASCII characters. The `ascii-forge` Rust library and CLI share their rendering and validation code with the browser editor.

[Open the browser editor](https://equitybot.github.io/ApollinairesASCII/) · [CLI guide](docs/CLI.md) · [Library example](examples/library.rs) · [Research and provenance](docs/RESEARCH.md)

```text
       .-.
      (o o)
      | O \
       \   \
        '~~~'
```

Every frame contains bytes 32 through 126. Line feeds separate rows when saved as text. The browser displays artwork in a `<pre>` using `textContent`. There are no image, SVG, canvas, Braille or Unicode block artwork outputs. Image files are accepted as conversion inputs.

## What it does

| Area | Included |
| --- | --- |
| Static art | 12 original templates, blank canvases, character editing, borders, crops, flips and clipped composition |
| Lettering | Standard, small, big, slant, block and outline styles; multiline text; alignment; plain FIGlet `.flf` import |
| Animation | Ghost, rotating donut, wave, starfield, rain, fire, text reveal and scrolling text; deterministic seeds |
| Media conversion | PNG, JPEG, GIF, WebP, BMP and PNM input; animated GIF timing; custom character ramps, aspect correction, transparency, inversion, gamma, contrast, dithering and edges |
| Browser editor | Select/copy text, timeline, per-frame timing, frame insertion/duplication/deletion/reordering, undo/redo and local file imports |
| Export | Plain `.txt`, timed `.json` projects and standalone `.html` text players |
| Native playback | Terminal playback with cursor movement; saved artwork remains plain text |

All maintained application logic is Rust, including the browser's DOM controls and file handling. HTML and CSS describe the interface. A small JavaScript loader starts WebAssembly; standalone HTML exports include a small JavaScript text player so each file works on its own.

## Run the CLI

Install [Rust](https://www.rust-lang.org/tools/install), then:

```sh
git clone https://github.com/EquityBot/ApollinairesASCII.git
cd ApollinairesASCII
cargo install --locked --path .
ascii-forge art ghost
ascii-forge text "RUST" --font block
ascii-forge animate ghost --output ghost.json
ascii-forge play ghost.json
ascii-forge image photo.png --width 80 --dither --output photo.txt
ascii-forge import ghost.json --format html --output ghost.html
```

The checked toolchain is pinned in `rust-toolchain.toml`. Existing output files are never overwritten. Static artwork defaults to text; animations default to JSON so frames and timings are retained. Use `--format text --frame 0` to deliberately select one animated frame.

## Use the library

The library has no browser or CLI dependency when default features are disabled:

```toml
[dependencies]
ascii-forge = { git = "https://github.com/EquityBot/ApollinairesASCII", default-features = false }

[patch.crates-io]
gif = { git = "https://github.com/image-rs/image-gif", rev = "e60e4dd7708cc50f9371185cefa04273f410f116" }
```

Include the patch in your application's workspace root whilst the upstream GIF fix awaits a stable release. Cargo does not inherit a dependency's patch section. The CLI and browser builds in this repository already apply it. See [dependency notes](docs/DEPENDENCIES.md).

```rust
use ascii_forge::{Animation, Canvas, SceneOptions, animation};

fn main() -> Result<(), String> {
    let mut canvas = Canvas::new(24, 6)?;
    canvas.rectangle(0, 0, 24, 6, b'#')?;
    canvas.text(3, 2, "HELLO, FRIEND")?;
    let still = Animation::still(canvas.finish());
    println!("{}", still.frames()[0].frame.text());

    let moving = animation::generate("ghost", &SceneOptions::default())?;
    let round_trip = Animation::from_json(&moving.to_json())?;
    assert_eq!(moving, round_trip);
    Ok(())
}
```

`Frame` is an immutable rectangular ASCII grid. `Canvas` provides validated drawing; `Animation` owns equal-size frames, durations and loop behaviour. See [the format and limits](docs/FORMAT.md) and run `cargo doc --no-deps --open` for the API.

## Build the browser editor

```sh
cargo build --locked --release --target wasm32-unknown-unknown --no-default-features --features web
cargo install --locked wasm-bindgen-cli --version 0.2.130
wasm-bindgen target/wasm32-unknown-unknown/release/ascii_forge.wasm --target web --out-dir web/pkg --out-name ascii_forge
```

Serve `web/` with a local HTTP server and open its URL. Opening `index.html` directly as a `file:` URL cannot load its WebAssembly module. GitHub Actions builds and deploys the same directory to GitHub Pages.

The editor processes uploads in your browser. It has no upload endpoint, account system, analytics or external font request. Playback starts paused when reduced motion is requested and pauses when the page is hidden. Save JSON to keep all frames and their timings; text export saves the selected frame.

## Checks

```sh
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked --release
cargo clippy --locked --target wasm32-unknown-unknown --no-default-features --features web -- -D warnings
```

The integration tests cover the ASCII contract, whitespace, drawing, all bundled styles and scenes, timing, round trips, malformed fonts, GIF preflight checks, resource limits, script-safe HTML and CLI file handling. CI runs the native checks on Linux and Windows and builds the browser module. See [validation](docs/VALIDATION.md) for the browser checks and known limits.

## Foundation and licence

ApollinairesASCII retains the history of [orhnk/RASCII](https://github.com/orhnk/RASCII), starting at `8d33a3670e51298d530ab202090b212501310cdd`. Its MIT image-conversion approach is the foundation of the converter. The shared frame model, drawing, scenes, project format, editor and exporters are new Rust code. It is an evolution of a focused converter, not a claim that upstream supplied this whole toolkit.

[Research](docs/RESEARCH.md) records 1,164 discovered repositories across 27 multilingual searches, the shortlist and why this foundation was selected. Discovery metadata is not a source audit of every result. No Ghostty animation artwork was copied.

MIT for this repository's code. Preserve the [licence](LICENSE.md) and [third-party notices](THIRD_PARTY_NOTICES.md). Dependencies and bundled font data retain their own licences and notices.

Hello, friend. Keep it in characters.
