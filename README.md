# ASUS (UM|M)5606 fan state script

Bash script to set the fan state on the ZenBook S 16 UM5606, Vivobook M5606, Zenbook S 14 UX5406SA and Vivobook S 15/16 S5506

## Usage

- `fan_state set <0-3|standard|quiet|high|full>`: set fan state to 0/standard, 1/quiet, 2/high, or 3/full
- `fan_state get`: gets current state
- `fan_state get-int`: gets current state as integer
- `fan_state help`: show help

## Dependencies

- Linux kernel 6.11+
- `bash`
- `dbus` (optional)

## Installation

- Arch Linux: [`asus-5606-fan-state-git` AUR package](https://aur.archlinux.org/packages/asus-5606-fan-state-git) (`yay -S asus-5606-fan-state-git`)
- Other Linux:
  - Copy `fan_state` to a directory in `$PATH`
  - Copy `asus-fan-permissions.service` to `/usr/lib/systemd/system/` & enable the service

## Shell Integration

If you happen to use [Noctalia Shell](https://noctalia.dev), there's a `fan_state` plugin on [my plugin repo](https://github.com/ThatOneCalculator/personal-noctalia-plugins).
