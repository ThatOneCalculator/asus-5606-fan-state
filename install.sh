#!/usr/bin/env bash

set -euo pipefail

echo "Installing asus-5606-fan-state..."

# Download the latest fan_state binary
echo "Downloading fan_state binary..."
curl -fsSL https://github.com/ThatOneCalculator/asus-5606-fan-state/releases/latest/download/fan_state -o fan_state
chmod +x ./fan_state

# Download the systemd service file
echo "Downloading systemd service file..."
curl -fsSL https://raw.githubusercontent.com/ThatOneCalculator/asus-5606-fan-state/refs/heads/main/asus-fan-permissions.service -o asus-fan-permissions.service

# Install the binary
echo "Installing fan_state to /usr/bin..."
sudo install -Dm755 fan_state /usr/bin/fan_state

# Install the systemd service
echo "Installing systemd service..."
sudo install -Dm644 asus-fan-permissions.service /usr/lib/systemd/system/asus-fan-permissions.service

# Reload systemd and enable the service
echo "Enabling and starting service..."
sudo systemctl daemon-reload
sudo systemctl enable --now asus-fan-permissions.service

# Clean up downloaded files
rm -f fan_state asus-fan-permissions.service

echo "Installation complete!"
