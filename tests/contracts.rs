use apollinaires_ascii::{
    Animation, Canvas, Frame, MAX_FRAMES, RenderOptions, SceneOptions, TimedFrame, animation, art,
    decode_media, export, lettering, render_image,
};
use image::{DynamicImage, Rgba, RgbaImage};
use lettering::Alignment;

fn ascii(frame: &Frame) {
    assert_eq!(frame.cells().len(), frame.width() * frame.height());
    assert!(frame.cells().iter().all(|b| (32..=126).contains(b)));
    assert!(
        frame
            .text()
            .bytes()
            .all(|b| b == b'\n' || (32..=126).contains(&b))
    );
}
fn timed(text: &str, ms: u32) -> TimedFrame {
    TimedFrame {
        frame: Frame::from_text(text).unwrap(),
        duration_ms: ms,
    }
}

#[test]
fn frame_refuses_every_control_and_non_ascii_byte() {
    for byte in 0..=255u8 {
        assert_eq!(
            Frame::new(1, 1, vec![byte]).is_ok(),
            (32..=126).contains(&byte),
            "byte {byte}"
        );
    }
    for input in [
        "",
        "a\tb",
        "a\rb",
        "a\r\nb",
        "a\u{1b}[31m",
        "café",
        "██",
        "👻",
        "aa\nb",
    ] {
        assert!(Frame::from_text(input).is_err(), "{input:?}");
    }
    assert!(Frame::new(usize::MAX, 2, vec![]).is_err());
    assert!(Frame::blank(0, 1).is_err());
    assert!(Frame::new(2, 2, vec![b' '; 3]).is_err());
}
#[test]
fn round_trips_preserve_whitespace_and_rectangular_shape() {
    for width in 1..20 {
        for height in 1..12 {
            let cells = (0..width * height)
                .map(|i| 32 + ((i * 37 + height) % 95) as u8)
                .collect();
            let frame = Frame::new(width, height, cells).unwrap();
            assert_eq!(Frame::from_text(&frame.text()).unwrap(), frame);
            assert_eq!(Frame::from_text(&(frame.text() + "\n")).unwrap(), frame);
            ascii(&frame);
        }
    }
    let padded = Frame::from_text_padded("a\nbbb\n").unwrap();
    assert_eq!(padded.text(), "a  \nbbb");
    assert_eq!(Frame::from_text("  \n  ").unwrap().text(), "  \n  ");
}
#[test]
fn frame_transforms_and_clipped_composition() {
    let frame = Frame::from_text("abc\ndef").unwrap();
    assert_eq!(frame.flip_horizontal().text(), "cba\nfed");
    assert_eq!(frame.flip_vertical().text(), "def\nabc");
    assert_eq!(frame.crop(1, 0, 2, 2).unwrap().text(), "bc\nef");
    assert!(frame.crop(usize::MAX, 0, 1, 1).is_err());
    assert_eq!(frame.border().unwrap().text(), "+---+\n|abc|\n|def|\n+---+");
    let mut canvas = Canvas::from_frame(&frame);
    canvas
        .overlay(&Frame::from_text(" X").unwrap(), 0, 0, true)
        .unwrap();
    assert_eq!(canvas.finish().text(), "aXc\ndef");
    let mut canvas = Canvas::new(3, 3).unwrap();
    canvas.line((-1, -1), (3, 3), b'#').unwrap();
    assert_eq!(canvas.finish().text(), "#  \n # \n  #");
    let mut canvas = Canvas::new(3, 3).unwrap();
    assert!(canvas.set(-1, -1, 0).is_err());
    assert!(canvas.line((i32::MIN, 0), (i32::MAX, 0), b'#').is_err());
    assert!(canvas.rectangle(i32::MAX, 0, 2, 2, b'#').is_err());
}
#[test]
fn animation_clock_uses_per_frame_durations_and_loop_policy() {
    let mut project = Animation::new(vec![timed("a", 10), timed("b", 30)], true).unwrap();
    for (time, index) in [
        (0, 0),
        (9, 0),
        (10, 1),
        (39, 1),
        (40, 0),
        (50, 1),
        (u64::MAX, 1),
    ] {
        assert_eq!(project.frame_at(time), index);
    }
    project.set_looping(false);
    assert_eq!(project.frame_at(40), 1);
    assert_eq!(project.frame_at(u64::MAX), 1);
    let original = project.clone();
    assert!(project.replace_frame(0, timed("x", 0)).is_err());
    assert_eq!(project, original);
    project.insert_frame(1, timed("c", 20)).unwrap();
    project.swap_frames(0, 2).unwrap();
    assert_eq!(project.frames()[0].frame.text(), "b");
    project.remove_frame(1).unwrap();
    assert!(project.remove_frame(9).is_err());
}
#[test]
fn project_validation_enforces_all_budgets() {
    assert!(Animation::new(vec![], true).is_err());
    assert!(Animation::new(vec![timed("x", 9)], true).is_err());
    assert!(Animation::new(vec![timed("x", 60_001)], true).is_err());
    assert!(Animation::new(vec![timed("x", 10), timed("xx", 10)], true).is_err());
    assert!(Animation::new(vec![timed("x", 10); MAX_FRAMES + 1], true).is_err());
    assert!(Animation::new(vec![timed("x", 60_000); 61], true).is_err());
    let big = TimedFrame {
        frame: Frame::blank(512, 256).unwrap(),
        duration_ms: 10,
    };
    assert!(Animation::new(vec![big; 62], true).is_err());
    let project = Animation::new(vec![timed("a ", 10), timed(" b", 20)], true).unwrap();
    assert_eq!(Animation::from_json(&project.to_json()).unwrap(), project);
    let mut value: serde_json::Value = serde_json::from_str(&project.to_json()).unwrap();
    for key in ["width", "height", "version", "frames", "loop", "format"] {
        let mut bad = value.clone();
        bad.as_object_mut().unwrap().remove(key);
        assert!(Animation::from_json(&bad.to_string()).is_err(), "{key}");
    }
    value["frames"][0]["text"] = serde_json::json!("é");
    assert!(Animation::from_json(&value.to_string()).is_err());
    value["frames"][0]["text"] = serde_json::json!("a ");
    value["unexpected"] = serde_json::json!(true);
    assert!(Animation::from_json(&value.to_string()).is_err());
}
#[test]
fn all_original_art_and_effects_are_real_ascii_and_deterministic() {
    for name in art::NAMES {
        ascii(&art::template(name).unwrap());
    }
    assert!(art::template("unknown").is_err());
    let options = SceneOptions {
        width: 36,
        height: 14,
        frames: 12,
        fps: 20,
        ..SceneOptions::default()
    };
    for scene in animation::SCENES {
        let a = animation::generate(scene, &options).unwrap();
        let b = animation::generate(scene, &options).unwrap();
        assert_eq!(a, b, "{scene}");
        for frame in a.frames() {
            ascii(&frame.frame);
        }
        assert!(
            a.frames().windows(2).any(|p| p[0].frame != p[1].frame),
            "{scene} must animate"
        );
        assert_eq!(Animation::from_json(&a.to_json()).unwrap(), a);
    }
    let mut other = options.clone();
    other.seed += 1;
    assert_ne!(
        animation::generate("starfield", &options).unwrap(),
        animation::generate("starfield", &other).unwrap()
    );
    other.fps = 0;
    assert!(animation::generate("ghost", &other).is_err());
    other.fps = 61;
    assert!(animation::generate("ghost", &other).is_err());
}
#[test]
fn lettering_covers_every_printable_ascii_character_in_every_font() {
    let chars: Vec<u8> = (32..=126).collect();
    for font in lettering::FONTS {
        for chunk in chars.chunks(8) {
            let text = std::str::from_utf8(chunk).unwrap();
            ascii(&lettering::render(text, font, None, Alignment::Left, None).unwrap());
        }
    }
    for text in ["👻", "é", "\t", "\r", ""] {
        assert!(lettering::render(text, "standard", None, Alignment::Left, None).is_err());
    }
    let narrow = lettering::render("A", "small", None, Alignment::Left, None).unwrap();
    let aligned = lettering::render(
        "A",
        "small",
        None,
        Alignment::Right,
        Some(narrow.width() + 4),
    )
    .unwrap();
    assert!(aligned.text().split('\n').all(|r| r.starts_with("    ")));
    assert!(lettering::render("A", "small", None, Alignment::Left, Some(1)).is_err());
}
fn simple_font() -> String {
    let mut font = String::from("flf2a$ 2 2 4 -1 0\n");
    for byte in 32..=126u8 {
        let c = byte as char;
        let marker = if c == '@' { '#' } else { '@' };
        font.push_str(&format!("{c}{marker}\n{c}{marker}{marker}\n"));
    }
    font
}
#[test]
fn custom_fonts_have_bounded_complete_ascii_glyphs() {
    ascii(&lettering::render("AB", "custom", Some(&simple_font()), Alignment::Left, None).unwrap());
    for header in [
        "",
        "flf2a$ 0 0 4 -1 0",
        "flf2a$ -1 0 4 -1 0",
        "flf2a$ 2 2 4 -1 -1",
        "flf2a👻 2 2 4 -1 0",
    ] {
        assert!(lettering::render("A", "custom", Some(header), Alignment::Left, None).is_err());
    }
    let font = simple_font().replace("A@\n", "é@\n");
    assert!(lettering::render("A", "custom", Some(&font), Alignment::Left, None).is_err());
}

#[test]
fn malformed_font_row_terminators_are_refused() {
    let malformed = simple_font().replace("A@\n", "A\n");
    assert!(lettering::render("A", "custom", Some(&malformed), Alignment::Left, None).is_err());
}

#[test]
fn gif_rectangles_must_fit_the_logical_screen_before_decode() {
    let mut bytes = gif_bytes(2);
    bytes[6..8].copy_from_slice(&1u16.to_le_bytes());
    bytes[8..10].copy_from_slice(&1u16.to_le_bytes());
    assert!(
        decode_media(
            &bytes,
            &RenderOptions {
                width: 1,
                height: Some(1),
                ..RenderOptions::default()
            }
        )
        .is_err()
    );
}

#[test]
fn duplicate_project_fields_are_rejected() {
    let json = Animation::still(Frame::from_text("a").unwrap()).to_json();
    let duplicate = format!("{{\"version\":999,{}", &json[1..]);
    assert!(Animation::from_json(&duplicate).is_err());
}

#[test]
fn imports_reject_large_or_malformed_structures_without_building_a_json_tree() {
    let project = Animation::still(Frame::from_text("a").unwrap()).to_json();
    let duplicate = project.replace(
        "\"duration_ms\":100",
        "\"duration_ms\":100,\"duration_ms\":200",
    );
    assert!(Animation::from_json(&duplicate).is_err());
    assert!(Animation::from_json(&project.replace("\"text\":\"a\"", "\"text\":[0,0,0]")).is_err());
    let too_many = format!(
        "{{\"format\":\"asciiforge\",\"version\":1,\"width\":1,\"height\":1,\"loop\":false,\"frames\":[{}]}}",
        vec!["{\"text\":\"a\",\"duration_ms\":100}"; 601].join(",")
    );
    assert!(Animation::from_json(&too_many).is_err());
    assert!(Frame::from_text(&"\n".repeat(apollinaires_ascii::MAX_INPUT_BYTES)).is_err());
    assert!(Frame::from_text_padded(&"\n".repeat(apollinaires_ascii::MAX_INPUT_BYTES)).is_err());
}

#[test]
fn custom_font_endmarks_can_vary_per_glyph_and_have_one_or_two_copies() {
    let mut font = String::from("flf2a$ 1 1 4 -1 0\n");
    for byte in 32..=126u8 {
        let c = char::from(byte);
        let marker = if c == '@' { '#' } else { '@' };
        font.push_str(&format!("{c}{marker}{marker}\n"));
    }
    let frame = lettering::render("A@B", "custom", Some(&font), Alignment::Left, None).unwrap();
    assert_eq!(frame.text(), "A@B");
    let uneven = simple_font().replace("A@@\n", "AA@@\n");
    assert!(lettering::render("A", "custom", Some(&uneven), Alignment::Left, None).is_err());
}
#[test]
fn image_mapping_handles_black_white_and_transparency_without_non_finite_normalisation() {
    let image = DynamicImage::ImageRgba8(RgbaImage::from_fn(4, 1, |x, _| match x {
        0 => Rgba([0, 0, 0, 255]),
        1 => Rgba([255, 255, 255, 255]),
        2 => Rgba([255, 255, 255, 0]),
        _ => Rgba([255, 255, 255, 128]),
    }));
    let options = RenderOptions {
        width: 4,
        height: Some(1),
        ..RenderOptions::default()
    };
    assert_eq!(render_image(&image, &options).unwrap().text(), " @ +");
    let inverted = RenderOptions {
        invert: true,
        ..options.clone()
    };
    assert_eq!(render_image(&image, &inverted).unwrap().text(), "@ @=");
    let white_bg = RenderOptions {
        background: 255,
        ..options.clone()
    };
    assert_eq!(render_image(&image, &white_bg).unwrap().cells()[2], b'@');
    let black = DynamicImage::ImageRgba8(RgbaImage::from_pixel(3, 2, Rgba([0, 0, 0, 255])));
    assert!(
        render_image(&black, &options)
            .unwrap()
            .cells()
            .iter()
            .all(|b| *b == b' ')
    );
    assert!(render_image(&DynamicImage::new_rgba8(0, 0), &options).is_err());
}
#[test]
fn every_image_mode_obeys_ascii_and_rejects_invalid_options() {
    let image = DynamicImage::ImageRgba8(RgbaImage::from_fn(16, 8, |x, y| {
        Rgba([(x * 16) as u8, (y * 32) as u8, 80, 255])
    }));
    for dither in [false, true] {
        for edges in [false, true] {
            for invert in [false, true] {
                ascii(
                    &render_image(
                        &image,
                        &RenderOptions {
                            width: 20,
                            dither,
                            edges,
                            invert,
                            ..RenderOptions::default()
                        },
                    )
                    .unwrap(),
                );
            }
        }
    }
    for ramp in ["", "a", " ░█", "a\nb", "\x1b#"] {
        assert!(
            render_image(
                &image,
                &RenderOptions {
                    ramp: ramp.into(),
                    ..RenderOptions::default()
                }
            )
            .is_err()
        );
    }
    for gamma in [f64::NAN, f64::INFINITY, 0.0, 6.0] {
        assert!(
            render_image(
                &image,
                &RenderOptions {
                    gamma,
                    ..RenderOptions::default()
                }
            )
            .is_err()
        );
    }
    assert!(decode_media(b"not an image", &RenderOptions::default()).is_err());
    assert!(decode_media(b"P6\n5000 1\n255\n", &RenderOptions::default()).is_err());
}
fn gif_bytes(count: usize) -> Vec<u8> {
    let mut bytes = Vec::new();
    {
        let mut encoder = image::codecs::gif::GifEncoder::new(&mut bytes);
        let frames = (0..count).map(|i| {
            image::Frame::from_parts(
                RgbaImage::from_pixel(
                    2,
                    2,
                    if i % 2 == 0 {
                        Rgba([255, 255, 255, 255])
                    } else {
                        Rgba([0, 0, 0, 255])
                    },
                ),
                0,
                0,
                image::Delay::from_numer_denom_ms(if i % 2 == 0 { 20 } else { 70 }, 1),
            )
        });
        encoder.encode_frames(frames).unwrap();
    }
    bytes
}
#[test]
fn animated_gif_import_preserves_frames_delays_and_predecode_limits() {
    let options = RenderOptions {
        width: 2,
        height: Some(2),
        ..RenderOptions::default()
    };
    let project = decode_media(&gif_bytes(2), &options).unwrap();
    assert_eq!(project.frames().len(), 2);
    assert_eq!(project.frames()[0].duration_ms, 20);
    assert_eq!(project.frames()[1].duration_ms, 70);
    assert_eq!(project.frames()[0].frame.text(), "@@\n@@");
    assert_eq!(project.frames()[1].frame.text(), "  \n  ");
    assert!(decode_media(&gif_bytes(MAX_FRAMES + 1), &options).is_err());
    let mut huge = gif_bytes(2);
    huge[6..8].copy_from_slice(&4097u16.to_le_bytes());
    assert!(decode_media(&huge, &options).is_err());
    let bytes = gif_bytes(2);
    for length in 0..bytes.len() {
        assert!(
            decode_media(&bytes[..length], &options).is_err(),
            "truncation {length}"
        );
    }
}
#[test]
fn html_export_cannot_turn_artwork_into_markup_or_script() {
    let text = "</script><img src=x onerror=alert(1)>&\"";
    let html = export::html(&Animation::still(Frame::from_text(text).unwrap()));
    assert!(!html.contains(text));
    assert!(!html.contains("<img"));
    assert!(!html.contains("<canvas"));
    assert!(html.contains("\\u003c/script\\u003e"));
    assert!(html.contains("art.textContent"));
    assert_eq!(html.matches("</script>").count(), 2);
}
