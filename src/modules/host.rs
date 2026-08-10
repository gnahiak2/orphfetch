// src/modules/host.rs
use sysinfo::System;

pub fn get() -> String {
    System::host_name().unwrap_or_else(|| "Unknown".into())
}
