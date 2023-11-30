use std::{env, path::PathBuf};

pub fn home_path() -> Option<PathBuf> {
    if cfg!(windows) {
        std::env::var("USERPROFILE").map(|v| PathBuf::from(v)).ok()
    } else {
        match std::env::var("XDG_CONFIG_HOME") {
            Ok(v) => Some(PathBuf::from(v)),
            Err(_) => env::var("HOME").map(|v| PathBuf::from(v)).ok(),
        }
    }
}
