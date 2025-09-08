use std::process::Command;

pub fn get_active_vpn() -> Option<String> {
    // Utilise nmcli pour lister les connexions VPN actives
    let output = Command::new("sh")
        .arg("-c")
        .arg("nmcli connection show --active | grep vpn | awk '{print $1}' | head -n 1")
        .output()
        .ok()?;
    if !output.stdout.is_empty() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

