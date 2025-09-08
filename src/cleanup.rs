use std::process::Command;
use crate::commands;

pub struct WifiCleanupGuard {
    pub monitor_interface: String,
    pub active_vpn: Option<String>,
}

impl Drop for WifiCleanupGuard {
    fn drop(&mut self) {
        println!("Stopping monitor mode on {}...", self.monitor_interface);
        let _ = Command::new("sudo")
            .arg("airmon-ng")
            .arg("stop")
            .arg(&self.monitor_interface)
            .status();

        println!("Re-Enabling NetworkManager");
        if !commands::run_sudo_command("systemctl", &["restart", "NetworkManager"]) {
            eprintln!("Error: Failed to restart NetworkManager");
        }

        // TODO: Re-connect to wifi

        if let Some(vpn_name) = &self.active_vpn {
            println!("Re-enabling VPN: {}...", vpn_name);
            let _ = Command::new("nmcli")
                .arg("connection")
                .arg("up")
                .arg(vpn_name)
                .status();
        }
    }
}

