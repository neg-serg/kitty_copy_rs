use std::process::{Command, Stdio};
use std::io::Write;

fn main() {
    let output = Command::new("kitty")
        .args(&["@","get-text","--extent=screen"])
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute kitty @ get-text");

    if !output.status.success() {
        eprintln!("Error: kitty command failed");
        std::process::exit(1);
    }

    let raw = String::from_utf8_lossy(&output.stdout);
    let cleaned = strip_ansi_escapes(&raw);

    if let Err(e) = copy_to_clipboard(&cleaned) {
        eprintln!("Clipboard error: {}", e);
        std::process::exit(1);
    }
}

fn strip_ansi_escapes(text: &str) -> String {
    let re = regex::Regex::new(r"\x1B(?:\[[0-9;]*[JKmsu]|\(B)").unwrap();
    re.replace_all(text, "").to_string()
}

fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    if Command::new("wl-copy").stdin(Stdio::piped()).spawn().is_ok() {
        let mut child = Command::new("wl-copy")
            .stdin(Stdio::piped())
            .spawn()?;
        child.stdin.as_mut().unwrap().write_all(text.as_bytes())?;
        child.wait()?;
        println!("✅ Copied to clipboard (wl-copy)");
    } else if Command::new("xclip").output().is_ok() {
        let mut child = Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .stdin(Stdio::piped())
            .spawn()?;
        child.stdin.as_mut().unwrap().write_all(text.as_bytes())?;
        child.wait()?;
        println!("✅ Copied to clipboard (xclip)");
    } else {
        println!("📋 Clipboard tool not found. Printing text:");
        println!("{}", text);
    }
    Ok(())
}
