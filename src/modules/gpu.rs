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
            let lower = line.to_lowercase();
            lower.contains("vga compatible controller")
                || lower.contains("3d controller")
                || lower.contains("display controller")
        })
        .and_then(|line| line.split_once(": "))
        .map(|(_, gpu)| {
            gpu.replace("Intel Corporation ", "")
                .replace("Advanced Micro Devices, Inc. ", "")
                .replace("AMD/ATI ", "")
                .replace("NVIDIA Corporation ", "")
                .trim()
                .to_string()
        })
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
