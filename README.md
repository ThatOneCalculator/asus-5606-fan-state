# ASUS (UM|M)5606 fan state

Program to set the fan state on the ZenBook S 16 UM5606, Vivobook M5606, Zenbook S 14 UX5406SA and Vivobook S 15/16 S5506

## Usage

- `fan_state set <0-3|standard|quiet|high|full>`: set fan state to 0/standard, 1/quiet, 2/high, or 3/full
- `fan_state get`: gets current state
- `fan_state get-int`: gets current state as integer
- `fan_state help`: show help

## Dependencies

- Linux kernel 6.11+
- Rust compiler (if building from source)
- `dbus` (optional)

## Installation

### Arch Linux

```sh
yay -S asus-5606-fan-state # -git, -bin
```

### Other Linux

- From source:
  - `cargo build --release`
    - Append `--no-default-features` to disable dbus
  - Copy `./target/release/fan_state` to a directory in `$PATH`
- Prebuilt binary:
  - Download [the latest binary](https://github.com/ThatOneCalculator/asus-5606-fan-state/releases/download/v1.0.0/fan_state) and move it into a directory in `$PATH`
- Copy `asus-fan-permissions.service` to `/usr/lib/systemd/system/` & enable the service (needed for both methods)

## Shell Integration

If you happen to use [Noctalia Shell](https://noctalia.dev), there's a `fan_state` plugin in [my plugin repo](https://github.com/ThatOneCalculator/personal-noctalia-plugins).

For your own integrations, to monitor the fan state, you can either:
- Monitor the `$XDG_RUNTIME_DIR/fan_state` file
- Monitor the `dev.t1c.FanState` interface for a `StateSet` member: `dbus-monitor --session "type='signal',interface='dev.t1c.FanState'"`
