use sysinfo::System;

pub fn get() -> String {
    System::long_os_version().unwrap_or_else(|| "Unknown".to_string());
}
