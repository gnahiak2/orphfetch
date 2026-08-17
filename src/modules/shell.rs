// src/modules/shell.rs
use std::env;
use std::process::Command;

pub fn get() -> String {
    let shell = match env::var("SHELL") {
        Ok(shell) => shell,
        Err(_) => return "Unknown".to_string(),
    };

    let name = match shell.rsplit('/').next() {
        Some(name) => name,
        None => return "Unknown".to_string(),
    };

    let version = match Command::new(&shell).arg("--version").output() {
        Ok(output) => {
            let output = String::from_utf8_lossy(&output.stdout);

            output.lines().next().and_then(|line| extract_version(line))
        }
        Err(_) => None,
    };

    match version {
        Some(version) => format!("{name} {version}"),
        None => name.to_string(),
    }
}

fn extract_version(text: &str) -> Option<String> {
    text.split_whitespace()
        .find(|part| part.chars().next().is_some_and(|c| c.is_ascii_digit()))
        .map(|version| {
            version
                .trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '.')
                .to_string()
        })
}
