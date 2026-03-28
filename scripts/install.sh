#!/bin/bash
set -euo pipefail
# only for agent download
# root is required
if [ "$EUID" -ne 0 ]; then
  echo "This script must be run as root"
  exit 1
fi

URL="https://github.com/Subhodip1307/ferroscope/releases/download/v0.1.0/x86_64-unknown-linux-musl.tar.gz"
FILE="ferroscope.tar.gz"
echo "Downloading..."
cd /usr/local/bin
curl -L "$URL" -o "$FILE"

echo "Extracting..."
tar -xzf "$FILE"
[ -f "$FILE" ] && rm "$FILE"
# need to check the hash
echo "Setting Permissions" 
chmod +x ferroscope-agent
#need to write



# User Part 
USER_NAME="ferroscope"
GROUP_NAME="ferroscope"

echo "Setting up user and group..."

# 1. Create group if it doesn't exist
if getent group "$GROUP_NAME" > /dev/null 2>&1; then
    echo "Group '$GROUP_NAME' already exists"
else
    echo "Creating group '$GROUP_NAME'"
    sudo groupadd "$GROUP_NAME"
fi

# 2. Create user if it doesn't exist
if id "$USER_NAME" > /dev/null 2>&1; then
    echo "User '$USER_NAME' already exists"
else
    echo "Creating user '$USER_NAME'"
    sudo useradd -r -s /usr/sbin/nologin -g "$GROUP_NAME" "$USER_NAME"
fi

# 3. Final Checking
if id -nG "$USER_NAME" | grep -qw "$GROUP_NAME"; then
    echo "User verification done"
else
    echo "Adding user '$USER_NAME' to group '$GROUP_NAME'"
    sudo usermod -aG "$GROUP_NAME" "$USER_NAME"
fi

echo "Done."

# systemd part
SERVICE_NAME="ferr"
SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"

sudo tee "$SERVICE_FILE" > /dev/null <<EOF
[Unit]
Description=ferroscope
After=network.target
StartLimitIntervalSec=600
StartLimitBurst=6

[Service]
Restart=on-failure
RestartSec=10s

ExecStart=/usr/local/bin/ferroscope-agent
WorkingDirectory=/usr/local/bin

ConfigurationDirectory=ferroscope_agent
ProtectSystem=strict
ReadWritePaths=/etc/ferroscope_agent
NoNewPrivileges=true

User=ferroscope
Group=ferroscope
Restart=on-failure
[Install]
WantedBy=multi-user.target
EOF

# restart the systemd 
echo "Reloading systemd..."
systemctl daemon-reload
echo "Enabling service..."
systemctl enable "$SERVICE_NAME"
./ferroscope-agent
