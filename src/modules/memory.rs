// src/modules/memory.rs
use sysinfo::System;

pub fn get(system: &System) -> String {
    let used = system.used_memory();
    let total = system.total_memory();

    format!(
        "{:.2} GiB / {:.2} GiB",
        used as f64 / 1024_f64.powi(3),
        total as f64 / 1024_f64.powi(3)
    )
}
