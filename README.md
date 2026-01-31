# ASUS (UM|M)5606 fan state

Program to set the fan state on the ZenBook S 16 UM5606, Vivobook M5606, Zenbook S 14 UX5406SA and Vivobook S 15/16 S5506

## Usage

- `fan_state set <0-3|standard|quiet|high|full>`: set fan state to 0/standard, 1/quiet, 2/high, or 3/full
- `fan_state get`: gets current state
- `fan_state get-int`: gets current state as integer
- `fan_state help`: show help

## Dependencies

- Linux kernel 6.11+
- Rust compiler <sub>(if building from source)</sub>
- systemd, bash <sub>(for systemd service, can be easily reimplemented for other init systems)</sub>
- dbus <sub>(optional)</sub>

## Installation

### Arch Linux

```sh
yay -S asus-5606-fan-state # -git, -bin
```

### Other Linux

#### Prebuilt binary with install script:
```sh
curl -fsSL https://raw.githubusercontent.com/ThatOneCalculator/asus-5606-fan-state/refs/heads/main/install.sh | bash
```

#### From source:
```sh
git clone https://github.com/ThatOneCalculator/asus-5606-fan-state
cd asus-5606-fan-state/
cargo build --release # --no-default-features to disable dbus
sudo install -Dm755 ./target/release/fan_state /usr/bin/fan_state
sudo install -Dm644 ./asus-fan-permissions.service /usr/lib/systemd/system/asus-fan-permissions.service
sudo systemctl daemon-reload
sudo systemctl enable --now asus-fan-permissions.service
```

## Desktop/Programatic Integration

If you happen to use [Noctalia Shell](https://noctalia.dev), there's a `fan_state` plugin in [my plugin repo](https://github.com/ThatOneCalculator/personal-noctalia-plugins).

For your own integrations, to monitor the fan state, you can either:
- Monitor the `$XDG_RUNTIME_DIR/fan_state` file
- Monitor the `dev.t1c.FanState` interface for a `StateSet` member: `dbus-monitor --session "type='signal',interface='dev.t1c.FanState'"`
