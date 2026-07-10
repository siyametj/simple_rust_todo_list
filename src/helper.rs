// src/helper.rs

use std::process::Command;

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .expect("Failed to clear screen on windows!");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen Unix/Linux/Mac!");
    }
}
