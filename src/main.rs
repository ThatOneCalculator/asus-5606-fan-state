use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process;

use clap::{Parser, Subcommand};
use dbus::Message;
use dbus::blocking::Connection;
use dbus::channel::{Channel, Sender};

const DEBUGFS_PATH: &str = "/sys/kernel/debug/asus-nb-wmi";

#[derive(Debug, Clone, Copy, PartialEq)]
enum FanState {
    Standard,
    Quiet,
    High,
    Full,
}

impl FanState {
    fn from_value(v: u8) -> Option<Self> {
        match v {
            0 => Some(FanState::Standard),
            1 => Some(FanState::Quiet),
            2 => Some(FanState::High),
            3 => Some(FanState::Full),
            _ => None,
        }
    }

    fn from_hex_str(s: &str) -> Option<Self> {
        match s.trim() {
            "0x00000000" => Some(FanState::Standard),
            "0x00000001" => Some(FanState::Quiet),
            "0x00000002" => Some(FanState::High),
            "0x00000003" => Some(FanState::Full),
            _ => None,
        }
    }

    fn as_u8(self) -> u8 {
        match self {
            FanState::Standard => 0,
            FanState::Quiet => 1,
            FanState::High => 2,
            FanState::Full => 3,
        }
    }

    fn name(self) -> &'static str {
        match self {
            FanState::Standard => "standard",
            FanState::Quiet => "quiet",
            FanState::High => "high",
            FanState::Full => "full",
        }
    }
}

/// A custom parser that accepts both names and numeric values
fn parse_fan_state(s: &str) -> Result<FanState, String> {
    if let Ok(n) = s.parse::<u8>() {
        return FanState::from_value(n)
            .ok_or_else(|| format!("Invalid fan state: {}. Use 0-3.", n));
    }

    match s.to_lowercase().as_str() {
        "standard" => Ok(FanState::Standard),
        "quiet" => Ok(FanState::Quiet),
        "high" => Ok(FanState::High),
        "full" => Ok(FanState::Full),
        _ => Err(format!(
            "Invalid fan state: '{}'. Use 0-3 or standard/quiet/high/full.",
            s
        )),
    }
}

#[derive(Parser)]
#[command(name = "fan_state")]
#[command(about = "ASUS laptop fan state control utility", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set the fan state
    #[command(visible_alias = "s")]
    Set {
        /// Fan state: 0/standard, 1/quiet, 2/high, 3/full
        #[arg(value_parser = parse_fan_state)]
        state: FanState,
    },

    /// Get the current fan state
    #[command(visible_alias = "g")]
    Get {
        /// Output as integer only
        #[arg(short, long)]
        int: bool,
    },
}

fn check_debugfs() -> Result<(), String> {
    let mounts = fs::read_to_string("/proc/mounts")
        .map_err(|e| format!("Failed to read /proc/mounts: {}", e))?;

    if !mounts.contains("debugfs") {
        return Err("Error: debugfs not mounted. Please mount it with:\n  \
             sudo mount -t debugfs none /sys/kernel/debug\n\
             Or add to /etc/fstab: debugfs /sys/kernel/debug debugfs mode=0755 0 0"
            .to_string());
    }

    if !Path::new(DEBUGFS_PATH).is_dir() {
        return Err(format!(
            "Error: {} not found.\n\
             Make sure the asus-nb-wmi module is loaded, and that the \
             asus-fan-permissions service is loaded and running.",
            DEBUGFS_PATH
        ));
    }

    let dev_id_path = format!("{}/dev_id", DEBUGFS_PATH);
    if File::options().write(true).open(&dev_id_path).is_err() {
        return Err(
            "Error: No write permission to /sys/kernel/debug/asus-nb-wmi/\n\
             Please run the systemd service to set permissions:\n  \
             sudo systemctl start asus-fan-permissions.service"
                .to_string(),
        );
    }

    Ok(())
}

fn get_reg_no() -> &'static str {
    let model = fs::read_to_string("/sys/class/dmi/id/product_name")
        .unwrap_or_default()
        .trim()
        .to_string();

    if model.contains("Vivobook S 15 S5506") {
        "0x5002f"
    } else if model.contains("Zenbook S 16 UM5606")
        || model.contains("Vivobook M5606")
        || model.contains("Zenbook S 14 UX5406SA")
    {
        "0x110019"
    } else {
        "0x110019"
    }
}

fn write_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = File::options().write(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn get_xdg_runtime_dir() -> Option<String> {
    if let Ok(dir) = env::var("XDG_RUNTIME_DIR") {
        return Some(dir);
    }

    if unsafe { libc::geteuid() } == 0 {
        if let Ok(sudo_uid) = env::var("SUDO_UID") {
            return Some(format!("/run/user/{}", sudo_uid));
        }
    }

    None
}

fn get_dbus_address() -> Option<String> {
    if let Ok(addr) = env::var("DBUS_SESSION_BUS_ADDRESS") {
        Some(addr)
    } else if unsafe { libc::geteuid() } == 0 {
        env::var("SUDO_UID")
            .ok()
            .map(|uid| format!("unix:path=/run/user/{}/bus", uid))
    } else {
        None
    }
}

fn send_dbus_signal(state: FanState) -> Result<(), Box<dyn std::error::Error>> {
    let Some(addr) = get_dbus_address() else {
        return Ok(());
    };

    let channel = Channel::open_private(&addr)?;
    let connection: Connection = channel.into();

    let signal = Message::signal(
        &"/dev/t1c/FanState".into(),
        &"dev.t1c.FanState".into(),
        &"StateSet".into(),
    )
    .append1(state.as_u8() as i32);

    connection
        .send(signal)
        .map_err(|_| "Failed to send D-Bus signal")?;

    Ok(())
}

fn set_fan_state(state: FanState) -> Result<(), String> {
    check_debugfs()?;

    println!("Setting fan state to {}", state.as_u8());

    let dev_id_path = format!("{}/dev_id", DEBUGFS_PATH);
    let ctrl_param_path = format!("{}/ctrl_param", DEBUGFS_PATH);
    let devs_path = format!("{}/devs", DEBUGFS_PATH);

    write_file(&dev_id_path, get_reg_no()).map_err(|e| format!("Failed to write dev_id: {}", e))?;

    write_file(&ctrl_param_path, &format!("{}", state.as_u8()))
        .map_err(|e| format!("Failed to write ctrl_param: {}", e))?;

    let devs = read_file(&devs_path).map_err(|e| format!("Failed to read devs: {}", e))?;
    print!("{}", devs);

    if let Some(ref dir) = get_xdg_runtime_dir() {
        if Path::new(dir).is_dir() {
            let state_file = format!("{}/fan_state", dir);
            if let Err(e) = fs::write(&state_file, format!("{}", state.as_u8())) {
                eprintln!("Warning: Failed to write fan state file: {}", e);
            }
        }
    }

    if let Err(e) = send_dbus_signal(state) {
        eprintln!("Warning: Failed to send D-Bus signal: {}", e);
    }

    Ok(())
}

fn get_fan_state(as_int: bool) -> Result<(), String> {
    check_debugfs()?;

    let dev_id_path = format!("{}/dev_id", DEBUGFS_PATH);
    let ctrl_param_path = format!("{}/ctrl_param", DEBUGFS_PATH);

    write_file(&dev_id_path, get_reg_no()).map_err(|e| format!("Failed to write dev_id: {}", e))?;

    let state_hex =
        read_file(&ctrl_param_path).map_err(|e| format!("Failed to read ctrl_param: {}", e))?;

    let state_hex = state_hex.trim();

    if as_int {
        if let Some(state) = FanState::from_hex_str(state_hex) {
            println!("{}", state.as_u8());
        } else {
            // Try to extract any digits from unknown state
            let num: String = state_hex.chars().filter(|c| c.is_ascii_digit()).collect();
            println!("{}", num);
        }
    } else if let Some(state) = FanState::from_hex_str(state_hex) {
        println!("{} ({})", state.name(), state.as_u8());
    } else {
        println!("unknown ({})", state_hex);
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Set { state } => set_fan_state(state),
        Commands::Get { int } => get_fan_state(int),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        process::exit(1);
    }
}
