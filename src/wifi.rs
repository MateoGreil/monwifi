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

pub fn find_bssid(interface: &str, essid: &str) -> Option<String> {
    println!("Searching for BSSID on {}...", essid);
    match Command::new("iwlist")
        .arg(interface)
        .arg("scan")
        .output() {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            output_str.lines()
                .find(|line| line.contains(&format!("ESSID:\"{}\"", essid)))
                .and_then(|_| output_str.lines().find(|line| line.contains("Address:")))
                .map(|bssid_line| {
                    let bssid = bssid_line.split_whitespace().nth(4).unwrap_or("").to_string();
                    bssid.trim().to_string()
                })
        }
        Err(e) => {
            eprintln!("Error: Failed to execute iwlist command: {}", e);
            None
        }
    }
}
