// src/platform/mod.rs
#[cfg(target_os = "macos")]
pub fn gpu() -> String {
    "Apple GPU".into()
}

#[cfg(target_os = "linux")]
pub fn gpu() -> String {
    "Unknown".into()
}

#[cfg(target_os = "windows")]
pub fn gpu() -> String {
    "Unknown".into()
}
