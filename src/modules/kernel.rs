// src/modules/kernel.rs
use sysinfo::System;

pub fn get() -> String {
    System::kernel_version().unwrap_or_else(|| "Unknown".into())
}
