# CLI guide

`apollinaire --help` lists commands. Each command also accepts `--help`.

```sh
apollinaire list
apollinaire art cat --border --output cat.txt
apollinaire text "Hello" --font small --align center --width 72
apollinaire text "RUST" --font-file my-font.flf
apollinaire animate donut --width 72 --height 24 --frames 120 --fps 30 --output donut.json
apollinaire animate scroll --text "HELLO, FRIEND" --seed 42 --output scroll.html
apollinaire image photo.jpg --width 100 --ramp fine --gamma 1.2 --contrast 1.3 --dither
apollinaire image movie.gif --width 72 --output movie.json
apollinaire image logo.png --width 72 --edges --invert --background 255
apollinaire import art.txt --pad --format json --output art.json
apollinaire import movie.json --format html --output movie.html
apollinaire import movie.json --format text --frame 0 --output first.txt
apollinaire compose background.txt cat.txt --x 4 --y 2 --transparent-spaces
apollinaire transform movie.json horizontal --output flipped.json
apollinaire transform art.txt crop --x 2 --y 1 --width 20 --height 10
apollinaire transform art.txt border
apollinaire check movie.json
apollinaire play movie.json --loops 3
```

`--output` creates a new file. An existing file causes an error, even when its contents are identical. Without it, exports go to stdout. Errors go to stderr and return a nonzero exit status. `check` reports validated dimensions and timing on stdout.

`--format text|json|html` overrides extension/default inference. `.json` and `.html` output extensions select those formats; otherwise a still defaults to text and an animation to JSON. Animated text export requires an explicit zero-based `--frame`. `--frame` is valid only with text export.

Use `-` as the input path for stdin. `.json` input is always parsed as a project and `.txt` as text. Ambiguous input, including stdin, is treated as JSON when it begins with `{` and is syntactically JSON; otherwise it is text. Give literal artwork that is also valid JSON a `.txt` filename to force text interpretation. CRLF is normalised to LF for text imports. `--pad` explicitly pads ragged rows; the browser pads text imports automatically.

`image` accepts a named ramp (`standard`, `fine`, `simple`, `binary`) or a literal sequence ordered from light to dense. Quote literal ramps containing spaces or shell characters. `--cell-aspect` defaults to `0.5`; use `--height` to set rows explicitly. The background is a greyscale value from 0 to 255 for compositing transparency. Gamma and contrast accept 0.1 to 5.0. Animated GIF input retains each frame's delay, normalising zero delay to 100 ms and applying the format's 10 to 60,000 ms limits.

`play` requires a terminal and refuses redirected stdout. It uses ANSI clear/cursor commands solely to update terminal text. It does not put those codes in the project or any text export. It plays the requested number of cycles; the project's loop setting controls the browser and standalone player. Unsupported media and malformed input fail visibly.
