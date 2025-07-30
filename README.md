# ASUS (UM|M)5606 fan state script

Bash script to set the fan state on the ZenBook S 16 UM5606, Vivobook M5606, and Zenbook S 14 UX5406SA

Note: for the Vivobook S 16 S5506, edit `fan_state` and replace all instances of `0x110019` with `0x5002f`

## Usage

- `fan_state get`: gets current state
- `fan_state set <0-3|standard|quiet|high|full>`: set fan state to 0/standard, 1/quiet, 2/high, or 3/full

## Dependencies

- Linux kernel 6.11+
- `bash`
- `sudo` (or sudo shim with `doas`)

## Installation

- Arch Linux: [`asus-5606-fan-state-git` AUR package](https://aur.archlinux.org/packages/asus-5606-fan-state-git) (`yay -S asus-5606-fan-state-git`)
- Other Linux: copy `fan_state` to a directory in `$PATH`, `chmod +x` if needed.
