// src/models/uptime.rs
use sysinfo::System;

pub fn get() -> String {
    let seconds = System::uptime();

    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;

    if hours > 0 {
        format!("{hours}h {minutes}m")
    } else {
        format!("{minutes}m")
    }
}
