use clap::Parser;
mod cleanup;
mod wifi;
mod vpn;
mod commands;

/// Get the WIFI password
#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Add author ?
struct Args {
    /// ESSID of the WIFI that you want to get the passsword.
    essid: String,

    /// Path to the dictionary
    #[arg(short, long)]
    dictionnary_path: Option<String>
}

fn main() {
    let args = Args::parse();
    let required_tools = ["NetworkManager", "airmon-ng", "airodump-ng", "aircrack-ng", "iwlist", "nmcli"];
    let interface = match wifi::get_wifi_interface() {
        Some(iface) => iface,
        None => {
            eprintln!("Error: No Wi-Fi interface found.");
            std::process::exit(1);
        }
    };
    let monitor_interface = format!("{}mon", interface);
    let active_vpn = vpn::get_active_vpn();
    let _wifi_cleanup_guard = cleanup::WifiCleanupGuard {
        monitor_interface: monitor_interface.clone(),
        active_vpn: active_vpn,
    };

    for tool in required_tools.iter() {
        if !commands::is_tool_installed(tool) {
            eprintln!("Error: {} is not installed. Please install it.", tool);
            return;
        }
    }

    let bssid = wifi::find_bssid(&interface, &args.essid);
    if bssid.is_none() {
        eprintln!("Error: BSSID not found.");
        return;
    }
    println!("BSSID Found {:?}!", bssid);
    return;


    if !commands::run_sudo_command("airmon-ng", &["check", "kill"]) {
        eprintln!("Error: Failed to run 'sudo airmon-ng check kill'");
        return;
    }

    println!("Starting monitor mode on {}...", monitor_interface);
    if !commands::run_sudo_command("airmon-ng", &["start", &interface]) {
        eprintln!("Error: Failed to start monitor mode on {}", interface);
        return;
    }
}
