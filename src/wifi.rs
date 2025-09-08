use std::process::Command;

pub fn get_wifi_interface() -> Option<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("iwconfig 2>/dev/null | grep -E '^[a-zA-Z0-9]+' | awk '{print $1}' | head -n 1")
        .output()
        .expect("Failed to execute command");
    if !output.stdout.is_empty() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

