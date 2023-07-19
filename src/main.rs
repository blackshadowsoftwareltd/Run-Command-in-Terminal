#![deny(clippy::all)]
use std::process::Command;

fn main() {
    let flutter_command = if cfg!(target_os = "windows") {
        "flutter.bat"
    } else {
        "flutter"
    };

    let project_path = "/home/remon/Rusty/";

    let output = Command::new(flutter_command)
        .current_dir(project_path)
        .args(&["create", "project_name"])
        .output()
        .expect("Failed to execute Flutter command");

    if output.status.success() {
        println!("Flutter project created successfully");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Failed to create Flutter project: {}", stderr);
    }
}
