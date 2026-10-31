# Validation and limits

Local validation on 1 November 2026 used Rust 1.99.1, Windows MSVC, Linux x86_64 and `wasm32-unknown-unknown`.

## Checks completed

| Check | Result |
| --- | --- |
| Linux `cargo build --locked`, `cargo test --locked`, Clippy across all native targets with warnings denied and formatting | Passed |
| Windows release build, native tests, Clippy across all native targets with warnings denied and formatting | Passed |
| Native tests | 17 contract tests, 3 CLI integration tests and 1 documentation example passed |
| Library example | Compiled and ran successfully |
| WebAssembly release build and Clippy with warnings denied | Passed with the `web` feature and default features disabled |
| wasm-bindgen | Generated browser bindings using pinned CLI version 0.2.130 |
| GitHub workflow syntax | Passed actionlint |
| Aikido source scan | Zero issues across 15 files; zero skipped paths |
| Aikido dependency scan | Reachable weezl alert fixed; 5 remaining transitive alerts documented in [dependency notes](DEPENDENCIES.md) |

Contract tests cover every control/non-ASCII byte, trailing spaces and rectangular parsing, drawing and transforms, all 95 printable characters across the 6 styles, original templates and scenes, deterministic seeds, per-frame timing, JSON validation and round trips, GIF timing/truncation/preflight bounds, image ramp modes, HTML injection strings and CLI output protection.

Independent code reviews found defects in JSON allocation, frame line allocation, GIF rectangles, font markers, ambiguous stdin, asynchronous edits, empty-frame editing and preview overflow. They were repaired and checked. The malformed-font, GIF-rectangle and duplicate-project-field tests reproduced their failures before repair; browser probes likewise reproduced editable-during-load, empty-edit rejection and negative tall-preview positioning before repair.

## Actual browser checks

Headless Brave loaded and executed the generated WebAssembly module over local HTTP. Checks passed for:

- Printable DOM text, text selection and absence of canvas/image/SVG artwork elements.
- Reduced-motion paused start, play/pause and frame stepping.
- All 12 static templates, 6 lettering styles and 8 procedural scenes.
- Frame editing, padding, timing, duplication, reordering, flips, add/delete, undo/redo and clearing a frame.
- Rejected Unicode and protection of unapplied edits.
- Pending file reads disabling draft editing and failed imports preserving the current artwork.
- Text, JSON and standalone HTML downloads, JSON reimport and execution of the standalone player.
- CRLF/ragged text import, custom FLF import, image conversion and 2-frame GIF import retaining 20/70 ms delays.
- Layouts at 360, 768 and 1,440 pixels, no page-level horizontal overflow and visible buttons at least 44 pixels high.
- Reachable top rows in a 256-row preview. Browser runtime errors: zero.
- Two complete non-looping playback cycles in the Rust editor and standalone HTML player, preserving 80/100 ms frame durations under a controlled browser clock, plus pause/resume retaining the remaining frame time.
- An actual unload prompt after a deliberately blocked JSON download. Download requests retain unsaved-work protection.

Screenshots were visually inspected. Clipboard permission was denied by the headless browser even after a test permission request. The visible select-and-copy fallback and actual text selection were checked; a successful clipboard API write was not verified in that environment.

These browser checks were local session checks, not a maintained cross-browser CI suite. CI compiles the browser and runs native contract tests. Firefox, Safari, touch hardware, screen-reader behaviour and macOS binaries have not been tested. Windows emits a Cargo PDB-name collision warning because the native library and CLI share a normalised crate name; the checks still pass. The local compiler-cache wrapper could not launch one large WebAssembly compile command, so local WASM checks were rerun successfully with that wrapper disabled.

## Deliberate limits

There is no video decoder, webcam capture, network media fetching, ANSI-colour artwork mode, raster/vector artwork exporter or Unicode-art mode. GIF is the animated media import format; other media inputs use the image library's still-image decoding. Oversize inputs are refused. Browser work is synchronous Rust after the bounded file read, so large valid conversions may briefly occupy the UI thread. Resource limits bound accepted files and projects, not the browser process's total memory.

The browser has bounded undo history and no automatic local persistence. Save a JSON project to retain all frames and timing. Text export saves one selected frame. Closing or reloading a modified project requests the browser's unsaved-work prompt; downloads are reported as requested because the page cannot prove where the browser saved a file. Requesting a download does not clear the unsaved-work warning.

Calvino design review completed with verified GPT-6 and an Extra High fallback after the model picker showed Pro unavailable. Final publication review and hosted CI/deployment results are recorded in the publication handoff; local checks alone do not establish those external results.
