# ASUS (UM|M)5606/5506 Fan State Control Script

Bash script to **view, set, and persist** the fan state on the following ASUS laptops:

- **ZenBook S 16 UM5606**
- **Vivobook M5606**
- **Zenbook S 14 UX5406SA**
- **Vivobook S 15 S5506**
- **Vivobook S 16 S5506**

***

## Features

- **View** current fan state.
- **Set fan state instantly** (Standard / Quiet / High / Full).
- **Make the fan state persistent** across reboots and wake from sleep using `systemd`.
- **Interactive menu mode** if run with no arguments.
- **Command-line mode** for scripting/automation.

***

## Usage

Run **without arguments** for interactive menu:

```bash
sudo fan_state
```

Command-line (CLI) usage:

```bash
# Show current fan state
sudo fan_state get

# Set fan state instantly
sudo fan_state set <0-3|standard|quiet|high|full>

# Set fan state and persist across reboots
sudo fan_state set-persistent <0-3|standard|quiet|high|full>
```


### Fan State Mapping

| Mode Name | Number | Description |
| :-- | :-- | :-- |
| `standard` | 0 | Balanced cooling |
| `quiet` | 1 | Lower noise, less cooling |
| `high` | 2 | High-performance mode |
| `full` | 3 | Maximum cooling performance |


***

## Dependencies

- **Linux kernel 6.11+** (with working `asus-nb-wmi` driver)
- `bash` shell
- Root privileges (`sudo` or root user)
- `systemd` (for persistent mode)

***

## Installation

### Arch Linux:

Install from the AUR:

```bash
yay -S asus-5606-fan-state-git
```


### Manual:

1. Clone the repository

```bash
git clone https://github.com/ThatOneCalculator/asus-5606-fan-state
```

2. Make it executable:
```bash
cd asus-5606-fan-state && chmod +x fan_state
```
3. Run the script:

```
sudo ./fan_state
```


***

## Persistent Mode Details

When using:

```bash
sudo fan_state set-persistent <mode>
```

The script:

- Sets the fan state immediately.
- Creates a **systemd service** (`/etc/systemd/system/fan-control.service`) that automatically applies the fan state at boot and after wake from sleep.

You can manage the service manually:

```bash
sudo systemctl disable fan-control.service
sudo systemctl enable fan-control.service
sudo systemctl start fan-control.service
sudo systemctl stop fan-control.service
```

***

## License

MIT License — free to use and modify.

***