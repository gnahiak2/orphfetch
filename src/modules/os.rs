// src/modules/os.rs
use sysinfo::System;

pub fn get() -> String {
    System::long_os_version().unwrap_or_else(|| "Unknown".into())
}
