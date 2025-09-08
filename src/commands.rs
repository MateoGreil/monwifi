use std::process::Command;

pub fn is_tool_installed(tool: &str) -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {}", tool))
        .output()
        .expect("Failed to execute command");
    !output.stdout.is_empty()
}

pub fn run_sudo_command(command: &str, args: &[&str]) -> bool {
    let mut cmd = Command::new("sudo");
    cmd.arg(command);
    cmd.args(args);
    let status = cmd.status().expect("Failed to execute command");
    status.success()
}

