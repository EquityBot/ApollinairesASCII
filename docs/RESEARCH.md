# Research and provenance

Research captured on 1 November 2026. The goal was a Rust library, CLI and browser editor whose artwork always remains printable ASCII characters, covering static art, lettering and animation.

## Search scope

The [discovery index](research/repositories.json) contains 1,164 distinct GitHub repositories from 27 searches. Queries cover English, Chinese, Japanese, Korean, Russian, Portuguese, Spanish and German terms, including `字符画`, `アスキーアート`, `아스키 아트`, `ASCII арт`, `ASCII imagem` and Rust/WASM/editor/animation/FIGlet combinations. Some queries returned no matches. Results were bounded relevance pages, normally the first 30 or 100, then deduplicated.

This is a broad discovery sample, not every ASCII repository on GitHub. GitHub search caps, ranking, private repositories, untagged projects and changing results prevent an exhaustive census. The index is discovery metadata, not a claim that every candidate's source or licence was audited. Closely related candidates received deeper README, manifest, rendering-path or licence checks; metadata-only comparisons are labelled below.

## Selection criteria

The useful foundation needed character output, Rust compatibility, a practical route to a shared native/WASM core, clear reuse terms, manageable dependencies and code narrow enough to understand. Stars were not a quality score. A large terminal framework can be excellent in its own scope whilst making a poor browser library foundation.

| Repository | What the evidence showed | Decision |
| --- | --- | --- |
| [orhnk/RASCII](https://github.com/orhnk/RASCII) | Rust; MIT; compact resize/luminance/character-ramp converter. Source and licence inspected. | Selected conversion foundation; upstream history retained. |
| [FineFindus/artem](https://github.com/FineFindus/artem) | Rust image-to-ASCII tool, MPL-2.0. Manifest/README/licence checked. | Strong focused alternative; not an animation/editor foundation. |
| [peterbrittain/asciimatics](https://github.com/peterbrittain/asciimatics) | Python terminal animation framework, Apache-2.0. README/manifest/licence checked. | Broad capability reference; Python and terminal ownership conflict with a shared Rust/WASM core. |
| [CameronFoxly/Ascii-Motion](https://github.com/CameronFoxly/Ascii-Motion) | Browser animation editor, TypeScript; README describes mixed MIT and proprietary licensing despite MIT metadata. | Interaction reference only; no source copied. |
| [maxcurzi/tplay](https://github.com/maxcurzi/tplay) | Rust MIT terminal media player with FFmpeg/libmpv and other native/network integrations. Manifest and README checked. | Powerful media reference; native decoder stack is unsuitable for the browser core. |
| [yuanbohan/rs-figlet](https://github.com/yuanbohan/rs-figlet) | Rust Apache-2.0 FIGlet implementation with 4 bundled font styles. Source, manifest and font headers inspected. | Reused as the `figlet-rs` dependency behind bounded plain-font validation. |
| [cmatsuoka/figlet](https://github.com/cmatsuoka/figlet) | FIGlet implementation and font format specification. Specification and licence checked. | Format reference, especially glyph end markers. |
| [dominikwilkowski/cfonts](https://github.com/dominikwilkowski/cfonts) | Rust lettering, GPL-3.0 in repository metadata. | Lettering comparison; no source copied. |
| [ChrisBuilds/terminaltexteffects](https://github.com/ChrisBuilds/terminaltexteffects) | Python MIT effects engine, metadata/README comparison. | Effects reference; no source copied. |
| [sepandhaghighi/art](https://github.com/sepandhaghighi/art) / [patorjk/figlet.js](https://github.com/patorjk/figlet.js) | Python and JavaScript lettering/art, MIT metadata. | Breadth references; preserve Rust implementation. |
| [durdraw/durdraw](https://github.com/durdraw/durdraw) | Python BSD-3-Clause editor supporting animation, ANSI, Unicode and CP437, metadata/README comparison. | Useful editor comparison; its broader character/colour modes exceed this project's ASCII contract. |
| [hpjansson/chafa](https://github.com/hpjansson/chafa) / [cacalabs/libcaca](https://github.com/cacalabs/libcaca) | C terminal graphics libraries with broader terminal/image modes, metadata comparison. | Too much terminal-specific surface for this foundation. |
| [TheZoraiz/ascii-image-converter](https://github.com/TheZoraiz/ascii-image-converter) / [IonicaBizau/image-to-ascii](https://github.com/IonicaBizau/image-to-ascii) | Go and JavaScript converters, Apache-2.0/MIT metadata. | Focused conversion alternatives; no source copied. |
| [YunYouJun/char-dust](https://github.com/YunYouJun/char-dust) | Chinese-described browser converter, TypeScript/MIT metadata. | Non-English discovery comparison; no source copied. |

Some older or guessed repository paths returned 404. They were not counted as inspected source or used as foundations.

## Ghostty and GIVEAWAY-ASCII

[Ghostty's website source](https://github.com/ghostty-org/website/tree/464e0f71ed50e94ceee8df84a9bb687e48cc0392/terminals/home/animation_frames) contains text animation frames loaded by its terminal component. This is the behaviour AsciiForge follows: selectable text cells changing over time. The original Ghostty artwork was not copied; AsciiForge's ghost is a new procedural scene.

[arimecoa/GIVEAWAY-ASCII](https://github.com/arimecoa/GIVEAWAY-ASCII) renders its effect in a canvas. Character-shaped pixels in a canvas do not meet this project's selectable-text output requirement, so it was not selected.

## Reuse boundary

The starting commit is RASCII `8d33a3670e51298d530ab202090b212501310cdd`. The resize, weighted luminance and ramp-index approach in `src/image_renderer.rs` is adapted from it, with the MIT notice preserved. ASCII validation, alpha compositing, resource bounds, dithering, edges and timed GIF import extend that component.

The frame/canvas model, scenes, animation format, editing, CLI and browser controls are new Rust code. `figlet-rs` supplies its font renderer and unmodified bundled font data as a dependency. Other compared projects supplied no copied code or artwork. See [third-party notices](../THIRD_PARTY_NOTICES.md) for the retained notices and [dependency notes](DEPENDENCIES.md) for scan results and upstream constraints.

No single inspected repository supplied the full requested combination. The result combines a small licensed Rust foundation with a shared Rust character model rather than claiming that one upstream project was universally best.
