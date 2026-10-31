#![cfg(feature = "cli")]
use std::io::Write;
use std::process::Command;
fn cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ascii-forge"))
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn native_commands_emit_real_text_and_validate_input() {
    let art = cli(&["art", "ghost"]);
    assert!(art.status.success());
    assert!(
        art.stdout
            .iter()
            .all(|b| *b == b'\n' || (32..=126).contains(b))
    );
    assert!(!art.stdout.contains(&27));
    let text = cli(&["text", "Hello", "--font", "small"]);
    assert!(text.status.success());
    let unicode = cli(&["text", "👻"]);
    assert!(!unicode.status.success());
    assert!(unicode.stdout.is_empty());
    let animation = cli(&[
        "animate", "wave", "--frames", "3", "--width", "20", "--height", "8",
    ]);
    assert!(animation.status.success());
    let project =
        ascii_forge::Animation::from_json(std::str::from_utf8(&animation.stdout).unwrap()).unwrap();
    assert_eq!(project.frames().len(), 3);
    let ambiguous = cli(&["animate", "ghost", "--format", "text"]);
    assert!(!ambiguous.status.success());
    let selected = cli(&["animate", "ghost", "--format", "text", "--frame", "1"]);
    assert!(selected.status.success());
}
#[test]
fn output_files_are_never_overwritten_and_playback_refuses_redirection() {
    let path = std::env::temp_dir().join(format!("ascii-forge-cli-{}.txt", std::process::id()));
    std::fs::write(&path, "keep this").unwrap();
    let output = cli(&["art", "cat", "--output", path.to_str().unwrap()]);
    assert!(!output.status.success());
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "keep this");
    let play = cli(&["play", path.to_str().unwrap()]);
    assert!(!play.status.success());
    assert!(play.stdout.is_empty());
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn stdin_ascii_can_start_with_an_opening_brace() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ascii-forge"))
        .args(["import", "-"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"{ a }").unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"{ a }\n");
}
