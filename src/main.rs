use ascii_forge::{
    Animation, Canvas, Frame, MAX_INPUT_BYTES, RenderOptions, Result, SceneOptions, TimedFrame,
    animation, art, export, lettering,
};
use clap::{Parser, Subcommand, ValueEnum};
use std::{
    fs::File,
    io::{self, IsTerminal, Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};

#[derive(Parser)]
#[command(
    version,
    about = "Static art, lettering and animation as real ASCII text"
)]
struct Args {
    #[command(subcommand)]
    command: Command,
    /// Create this file. Existing files are never overwritten.
    #[arg(short, long, global = true)]
    output: Option<PathBuf>,
    /// Defaults to text for still art and JSON for animation.
    #[arg(long, global = true, value_enum)]
    format: Option<Format>,
    /// Export one zero-based frame when selecting text from an animation.
    #[arg(long, global = true)]
    frame: Option<usize>,
}
#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Text,
    Json,
    Html,
}
#[derive(Clone, Copy, ValueEnum)]
enum Align {
    Left,
    Center,
    Right,
}
#[derive(Clone, Copy, ValueEnum)]
enum Transform {
    Horizontal,
    Vertical,
    Border,
    Crop,
}
#[derive(Subcommand)]
enum Command {
    /// List artwork, fonts, scenes and conversion ramps.
    List,
    /// Original static ASCII templates.
    Art {
        name: String,
        #[arg(long)]
        border: bool,
    },
    /// ASCII block letters, including punctuation and multiline text.
    Text {
        text: String,
        #[arg(long, default_value = "standard")]
        font: String,
        #[arg(long)]
        font_file: Option<PathBuf>,
        #[arg(long, value_enum, default_value = "left")]
        align: Align,
        #[arg(long)]
        width: Option<usize>,
    },
    /// Convert an image or animated GIF to text.
    Image {
        input: PathBuf,
        #[arg(short, long, default_value_t = 80)]
        width: usize,
        #[arg(long)]
        height: Option<usize>,
        #[arg(long, default_value = "standard")]
        ramp: String,
        #[arg(long)]
        invert: bool,
        #[arg(long)]
        dither: bool,
        #[arg(long)]
        edges: bool,
        #[arg(long, default_value_t = 1.0)]
        gamma: f64,
        #[arg(long, default_value_t = 1.0)]
        contrast: f64,
        #[arg(long, default_value_t = 0.5)]
        cell_aspect: f64,
        #[arg(long, default_value_t = 0)]
        background: u8,
    },
    /// Deterministic procedural ASCII scenes, exported as timed JSON.
    Animate {
        #[arg(default_value = "ghost")]
        scene: String,
        #[arg(long, default_value_t = 72)]
        width: usize,
        #[arg(long, default_value_t = 24)]
        height: usize,
        #[arg(long, default_value_t = 80)]
        frames: usize,
        #[arg(long, default_value_t = 20)]
        fps: u32,
        #[arg(long, default_value_t = 42)]
        seed: u32,
        #[arg(long, default_value = "HELLO, FRIEND")]
        text: String,
    },
    /// Load .txt or an ApollinairesASCII .json project. Text CRLF is normalised to LF.
    Import {
        input: PathBuf,
        #[arg(long)]
        pad: bool,
    },
    /// Overlay one static text frame onto another.
    Compose {
        background: PathBuf,
        overlay: PathBuf,
        #[arg(long, default_value_t = 0)]
        x: i32,
        #[arg(long, default_value_t = 0)]
        y: i32,
        #[arg(long)]
        transparent_spaces: bool,
    },
    /// Transform every frame in a project.
    Transform {
        input: PathBuf,
        #[arg(value_enum)]
        operation: Transform,
        #[arg(long, default_value_t = 0)]
        x: usize,
        #[arg(long, default_value_t = 0)]
        y: usize,
        #[arg(long)]
        width: Option<usize>,
        #[arg(long)]
        height: Option<usize>,
    },
    /// Play a project in a terminal. Escape codes are playback transport only.
    Play {
        input: PathBuf,
        #[arg(long,default_value_t=1,value_parser=clap::value_parser!(u32).range(1..=1000))]
        loops: u32,
    },
    /// Validate an existing text or JSON project.
    Check { input: PathBuf },
}
fn main() {
    if let Err(error) = run(Args::parse()) {
        eprintln!("ascii-forge: {error}");
        std::process::exit(1);
    }
}
fn run(args: Args) -> Result<()> {
    let project = match args.command {
        Command::List => {
            println!(
                "Art: {}\nFonts: {}\nScenes: {}\nRamps: {}",
                art::NAMES.join(", "),
                lettering::FONTS.join(", "),
                animation::SCENES.join(", "),
                ascii_forge::image_renderer::RAMPS
                    .iter()
                    .map(|(n, _)| *n)
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            return Ok(());
        }
        Command::Art { name, border } => {
            let frame = art::template(&name)?;
            Animation::still(if border { frame.border()? } else { frame })
        }
        Command::Text {
            text,
            font,
            font_file,
            align,
            width,
        } => {
            let custom = font_file.as_ref().map(|p| read_text(p)).transpose()?;
            let alignment = match align {
                Align::Left => lettering::Alignment::Left,
                Align::Center => lettering::Alignment::Center,
                Align::Right => lettering::Alignment::Right,
            };
            Animation::still(lettering::render(
                &text,
                &font,
                custom.as_deref(),
                alignment,
                width,
            )?)
        }
        Command::Image {
            input,
            width,
            height,
            ramp,
            invert,
            dither,
            edges,
            gamma,
            contrast,
            cell_aspect,
            background,
        } => {
            let ramp = ascii_forge::image_renderer::RAMPS
                .iter()
                .find(|(n, _)| *n == ramp)
                .map_or(ramp.clone(), |(_, r)| r.to_string());
            ascii_forge::decode_media(
                &read_bounded(&input)?,
                &RenderOptions {
                    width,
                    height,
                    ramp,
                    invert,
                    dither,
                    edges,
                    gamma,
                    contrast,
                    cell_aspect,
                    background,
                },
            )?
        }
        Command::Animate {
            scene,
            width,
            height,
            frames,
            fps,
            seed,
            text,
        } => animation::generate(
            &scene,
            &SceneOptions {
                width,
                height,
                frames,
                fps,
                seed,
                text,
            },
        )?,
        Command::Import { input, pad } => load(&input, pad)?,
        Command::Compose {
            background,
            overlay,
            x,
            y,
            transparent_spaces,
        } => {
            let base = load(&background, true)?;
            let stamp = load(&overlay, true)?;
            if base.frames().len() != 1 || stamp.frames().len() != 1 {
                return Err("Compose needs 2 static frames".into());
            }
            let mut canvas = Canvas::from_frame(&base.frames()[0].frame);
            canvas.overlay(&stamp.frames()[0].frame, x, y, transparent_spaces)?;
            Animation::still(canvas.finish())
        }
        Command::Transform {
            input,
            operation,
            x,
            y,
            width,
            height,
        } => {
            let project = load(&input, false)?;
            let mut frames = Vec::with_capacity(project.frames().len());
            for entry in project.frames() {
                let frame = match operation {
                    Transform::Horizontal => entry.frame.flip_horizontal(),
                    Transform::Vertical => entry.frame.flip_vertical(),
                    Transform::Border => entry.frame.border()?,
                    Transform::Crop => entry.frame.crop(
                        x,
                        y,
                        width.ok_or("Crop needs --width")?,
                        height.ok_or("Crop needs --height")?,
                    )?,
                };
                frames.push(TimedFrame {
                    frame,
                    duration_ms: entry.duration_ms,
                });
            }
            Animation::new(frames, project.looping())?
        }
        Command::Play { input, loops } => {
            if args.output.is_some() || args.format.is_some() || args.frame.is_some() {
                return Err("Play does not accept export options".into());
            }
            return play(&load(&input, false)?, loops);
        }
        Command::Check { input } => {
            let project = load(&input, false)?;
            println!(
                "Valid ASCII: {} x {}, {} frame(s), {} ms",
                project.width(),
                project.height(),
                project.frames().len(),
                project.duration_ms()
            );
            return Ok(());
        }
    };
    let format = args.format.unwrap_or_else(|| {
        match args
            .output
            .as_ref()
            .and_then(|p| p.extension())
            .and_then(|s| s.to_str())
        {
            Some("json") => Format::Json,
            Some("html") => Format::Html,
            _ if project.frames().len() > 1 && args.frame.is_none() => Format::Json,
            _ => Format::Text,
        }
    });
    if args.frame.is_some() && !matches!(format, Format::Text) {
        return Err("--frame applies only to text output".into());
    }
    let content = match format {
        Format::Text => {
            if project.frames().len() > 1 && args.frame.is_none() {
                return Err("Animation needs JSON or HTML, or select --frame N for text".into());
            }
            let entry = project
                .frames()
                .get(args.frame.unwrap_or(0))
                .ok_or("Frame index is outside the project")?;
            format!("{}\n", entry.frame.text())
        }
        Format::Json => format!("{}\n", project.to_json()),
        Format::Html => export::html(&project),
    };
    if let Some(path) = args.output {
        let mut file = File::options()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| format!("Cannot create {}: {e}", path.display()))?;
        file.write_all(content.as_bytes())
            .map_err(|e| format!("Write failed: {e}"))?;
    } else {
        io::stdout()
            .lock()
            .write_all(content.as_bytes())
            .map_err(|e| format!("Output failed: {e}"))?;
    }
    Ok(())
}
fn read_bounded(path: &Path) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    let reader: Box<dyn Read> = if path == Path::new("-") {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(path).map_err(|e| format!("Cannot read {}: {e}", path.display()))?)
    };
    reader
        .take(MAX_INPUT_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("Read failed: {e}"))?;
    if bytes.len() > MAX_INPUT_BYTES {
        return Err("Input exceeds 16 MiB".into());
    }
    Ok(bytes)
}
fn read_text(path: &Path) -> Result<String> {
    String::from_utf8(read_bounded(path)?)
        .map_err(|_| "Text input must be UTF-8 containing printable ASCII".into())
}
fn load(path: &Path, pad: bool) -> Result<Animation> {
    let text = read_text(path)?;
    if path.extension().is_some_and(|e| e == "json")
        || (!path.extension().is_some_and(|e| e == "txt")
            && text.trim_start().starts_with('{')
            && serde_json::from_str::<serde::de::IgnoredAny>(&text).is_ok())
    {
        Animation::from_json(&text)
    } else {
        let text = text.replace("\r\n", "\n");
        Ok(Animation::still(if pad {
            Frame::from_text_padded(&text)?
        } else {
            Frame::from_text(&text)?
        }))
    }
}
fn play(project: &Animation, loops: u32) -> Result<()> {
    let stdout = io::stdout();
    if !stdout.is_terminal() {
        return Err(
            "Playback needs a terminal; export JSON or HTML when redirecting output".into(),
        );
    }
    let mut output = stdout.lock();
    output
        .write_all(b"\x1b[2J\x1b[H")
        .map_err(|e| e.to_string())?;
    for _ in 0..loops {
        for entry in project.frames() {
            write!(output, "\x1b[H{}", entry.frame.text()).map_err(|e| e.to_string())?;
            output.flush().map_err(|e| e.to_string())?;
            std::thread::sleep(Duration::from_millis(u64::from(entry.duration_ms)));
        }
    }
    writeln!(output).map_err(|e| e.to_string())
}
