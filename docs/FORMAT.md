# Text and project format

Artwork cells are printable ASCII bytes 32 through 126, inclusive. LF separates rows in serialised text. Tabs, CR, NUL, DEL, escape codes, accented characters, emoji and Unicode block characters are rejected. Text imports may normalise CRLF before calling the core parser. Trailing spaces are significant; no final LF is stored in `Frame::text()`.

Frames are nonempty rectangles. `Frame::from_text` requires equal-width rows. `Frame::from_text_padded` explicitly pads shorter rows on the right. Both permit one final LF. A blank canvas is a rectangle of spaces rather than an empty frame.

A version 1 JSON project looks like this:

```json
{"format":"asciiforge","version":1,"width":3,"height":1,"loop":true,"frames":[{"text":"o o","duration_ms":100},{"text":"- -","duration_ms":150}]}
```

All fields are required and no additional or duplicate fields are accepted. JSON property order is immaterial. Frame dimensions must match the project. Import validates typed fields and budgets during deserialisation rather than allocating an arbitrary JSON value tree. The immutable frame type prevents invalid artwork from entering a valid project through the public API.

| Limit | Maximum |
| --- | --- |
| Canvas | 512 columns by 256 rows |
| Frames | 600 |
| Total project cells | 8,000,000 |
| Frame duration | 10 to 60,000 ms |
| Project duration | One hour |
| Effect duration | 100 to 60,000 ms at 1 to 60 FPS |
| File input | 16 MiB |
| CLI URL input | `https://` only, 16 MiB, 30 seconds, 5 redirects, `image/` content type |
| Media dimensions | 4,096 per side, 16,777,216 pixels |
| Image decoder allocation limit | 128 MiB |
| GIF logical-screen pixels across frames | 64,000,000 |
| Custom font file | 512 KiB |
| Custom font height / row length | 32 rows / 256 bytes |
| Browser undo history | 20 states and 16,000,000 cells |

GIF headers, image rectangles, frame count, aggregate decoded screen pixels and output cell budget are inspected before frame decoding. Image rectangles must be nonzero and fit inside the logical screen. Decoder limits complement these checks; they are not a process-wide memory cap. The browser retains the current project, bounded history and temporary decode data, so peak memory can exceed the stored character count.

Plain FLF fonts must define all 95 printable ASCII glyphs with printable ASCII rows of equal width within each glyph and valid end markers. Markers may differ between glyphs. One or two markers per row are normalised for the font renderer. Extended codepoints and tagged glyphs are ignored because input lettering accepts printable ASCII only. Compressed font archives are not accepted. Rendering that exceeds canvas limits is refused.

JSON stores characters and timing, not pixels. HTML exports embed the JSON safely and put each frame into a `<pre>` using `textContent`; they contain a small standalone JavaScript player. Neither format includes raster or vector artwork. The player starts paused, supports stepping and pauses on a hidden page or a change to reduced motion.
