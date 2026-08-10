// src/modules/cpu/rs
use sysinfo::System;

pub fn get(system: &System) -> String {
    system
        .cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown".into())
}
