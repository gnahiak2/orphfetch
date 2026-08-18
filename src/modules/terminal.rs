// src/modules/terminal.rs
use std::env;

pub fn get() -> String {
    if let Ok(term) = env::var("TERM_PROGRAM") {
        return term;
    }

    if let Ok(term) = env::var("TERM") {
        return term;
    }

    "Unknown".to_string()
}
