use crate::{Frame, Result, dimensions, printable};
use figlet_rs::FIGlet;

pub const FONTS: &[&str] = &["standard", "small", "big", "slant", "block", "outline"];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

/// External fonts must be plain FLF text with all 95 ASCII glyphs.
pub fn render(
    text: &str,
    font: &str,
    custom: Option<&str>,
    alignment: Alignment,
    width: Option<usize>,
) -> Result<Frame> {
    if text.is_empty() || text.len() > 512 {
        return Err("Lettering needs 1..=512 ASCII characters".into());
    }
    for line in text.split('\n') {
        printable(line.as_bytes())?;
    }
    let parsed = if let Some(contents) = custom {
        FIGlet::from_content(&prepare_font(contents)?)?
    } else {
        match font {
            "standard" | "outline" => FIGlet::standard()?,
            "small" => FIGlet::small()?,
            "big" | "block" => FIGlet::big()?,
            "slant" => FIGlet::slant()?,
            _ => return Err(format!("Unknown font {font}; choose {}", FONTS.join(", "))),
        }
    };
    let mut rows = Vec::new();
    for line in text.split('\n') {
        let figure = parsed
            .convert(if line.is_empty() { " " } else { line })
            .ok_or("Font cannot render this text")?;
        let rendered = figure.as_str();
        rows.extend(
            rendered
                .trim_end_matches('\n')
                .split('\n')
                .map(str::to_owned),
        );
    }
    let mut frame = Frame::from_text_padded(&rows.join("\n"))?;
    if custom.is_none() && font == "block" {
        frame = Frame::new(
            frame.width(),
            frame.height(),
            frame
                .cells()
                .iter()
                .map(|b| if *b == b' ' { b' ' } else { b'#' })
                .collect(),
        )?;
    } else if custom.is_none() && font == "outline" {
        let mut cells = frame.cells().to_vec();
        for y in 0..frame.height() {
            for x in 0..frame.width() {
                if frame.cells()[y * frame.width() + x] == b' ' {
                    continue;
                }
                let boundary = x == 0
                    || y == 0
                    || x + 1 == frame.width()
                    || y + 1 == frame.height()
                    || [
                        (x.saturating_sub(1), y),
                        (x + 1, y),
                        (x, y.saturating_sub(1)),
                        (x, y + 1),
                    ]
                    .iter()
                    .any(|(px, py)| {
                        *px < frame.width()
                            && *py < frame.height()
                            && frame.cells()[py * frame.width() + px] == b' '
                    });
                cells[y * frame.width() + x] = if boundary { b'#' } else { b' ' };
            }
        }
        frame = Frame::new(frame.width(), frame.height(), cells)?;
    }
    let target = width.unwrap_or(frame.width());
    dimensions(target, frame.height())?;
    if target < frame.width() {
        return Err(format!(
            "Lettering is {} columns; choose a wider canvas or shorter text",
            frame.width()
        ));
    }
    let left = match alignment {
        Alignment::Left => 0,
        Alignment::Center => (target - frame.width()) / 2,
        Alignment::Right => target - frame.width(),
    };
    let rows = frame
        .text()
        .split('\n')
        .map(|row| {
            format!(
                "{}{}{}",
                " ".repeat(left),
                row,
                " ".repeat(target - left - frame.width())
            )
        })
        .collect::<Vec<_>>();
    Frame::from_text(&rows.join("\n"))
}
fn prepare_font(contents: &str) -> Result<String> {
    if contents.len() > 512 * 1024 {
        return Err("Font exceeds 512 KiB".into());
    }
    let mut lines = contents.lines();
    let header = lines.next().ok_or("Font is empty")?;
    let fields: Vec<&str> = header.split_whitespace().collect();
    if fields.len() < 6
        || fields[0].len() != 6
        || !fields[0].starts_with("flf2a")
        || !fields[0].is_ascii()
        || !(33..=126).contains(&fields[0].as_bytes()[5])
    {
        return Err("Expected a plain FIGlet FLF header".into());
    }
    let height: usize = fields[1].parse().map_err(|_| "Invalid font height")?;
    let max_length: usize = fields[3].parse().map_err(|_| "Invalid font width")?;
    let comments: usize = fields[5]
        .parse()
        .map_err(|_| "Invalid font comment count")?;
    if !(1..=32).contains(&height) || !(1..=256).contains(&max_length) || comments > 1024 {
        return Err("Font dimensions or comment count exceed limits".into());
    }
    let mut prepared = format!("{header}\n");
    for _ in 0..comments {
        prepared.push_str(lines.next().ok_or("Font comment lines are missing")?);
        prepared.push('\n');
    }
    // Import only printable ASCII glyphs. Extended codepoints cannot replace them.
    // Normalise valid one/two endmarks for figlet-rs's row parser.
    for _ in 0..95 {
        let mut marker = None;
        let mut width = None;
        for row in 0..height {
            let line = lines
                .next()
                .ok_or("Font must include all 95 printable ASCII glyphs")?;
            if line.len() > 256 {
                return Err("Font row exceeds 256 bytes".into());
            }
            printable(line.as_bytes())?;
            let trimmed = line.trim_end_matches(' ');
            let end = *trimmed
                .as_bytes()
                .last()
                .ok_or("Font glyph is missing an end marker")?;
            if marker.is_some_and(|previous| previous != end) {
                return Err("Font glyph rows must use the same end marker".into());
            }
            marker = Some(end);
            let body = trimmed.trim_end_matches(char::from(end));
            let count = trimmed.len() - body.len();
            if count > 2 || body.is_empty() || width.is_some_and(|previous| previous != body.len())
            {
                return Err("Font glyph rows need equal widths and one or two end markers".into());
            }
            width = Some(body.len());
            prepared.push_str(body);
            prepared.push(char::from(end));
            if row + 1 == height && height > 1 {
                prepared.push(char::from(end));
            }
            prepared.push('\n');
        }
    }
    Ok(prepared)
}
