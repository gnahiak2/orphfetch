// src/modules/gpu.rs
use std::process::Command;

pub fn get() -> String {
    #[cfg(target_os = "linux")]
    {
        return linux();
    }

    #[cfg(target_os = "macos")]
    {
        return macos();
    }

    #[allow(unreachable_code)]
    "Unknown".to_string()
}

#[cfg(target_os = "linux")]
fn linux() -> String {
    let output = match Command::new("lspci").output() {
        Ok(output) => output,
        Err(_) => return "Unknown".to_string(),
    };

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find(|line| {
            let line = line.to_lowercase();
            line.contains("vga compatible controller")
                || line.contains("3d controller")
                || line.contains("display controller")
        })
        .and_then(|line| line.split_once(':'))
        .map(|(_, gpu)| gpu.trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

#[cfg(target_os = "macos")]
fn macos() -> String {
    let output = match Command::new("system_profiler")
        .args(["SPDisplaysDataType"])
        .output()
    {
        Ok(output) => output,
        Err(_) => return "Unknown".to_string(),
    };

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("Chipset Model:")
                .map(str::trim)
                .map(String::from)
        })
        .unwrap_or_else(|| "Unknown".to_string())
}
