#!/bin/bash
set -euo pipefail
LOG_DIR="/var/log/ferroscope"
CONF_FILE="/etc/logrotate.d/ferroscope"
URL="https://github.com/Subhodip1307/ferroscope/releases/download/v2.2/x86_64-unknown-linux-musl.tar.gz"

# only for agent download
# root is required
if [ "$EUID" -ne 0 ]; then
  echo "This script must be run as root"
  exit 1
fi

# User Part
USER_NAME="ferroscope"
GROUP_NAME="ferroscope"

echo "Setting up user and group..."

# 1. Create group if it doesn't exist
if getent group "$GROUP_NAME" > /dev/null 2>&1; then
    echo "Group '$GROUP_NAME' already exists"
else
    echo "Creating group '$GROUP_NAME'"
    groupadd "$GROUP_NAME"
fi

# 2. Create user if it doesn't exist
if id "$USER_NAME" > /dev/null 2>&1; then
    echo "User '$USER_NAME' already exists"
else
    echo "Creating user '$USER_NAME'"
    useradd -r -s /usr/sbin/nologin -g "$GROUP_NAME" "$USER_NAME"
fi

# 3. Final Checking
if id -nG "$USER_NAME" | grep -qw "$GROUP_NAME"; then
    echo "User verification done"
else
    echo "Adding user '$USER_NAME' to group '$GROUP_NAME'"
    usermod -aG "$GROUP_NAME" "$USER_NAME"
fi

echo "Done."

# installing binary
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

echo "Downloading..."
curl -fL --proto '=https' "$URL" -o "$TMP/ferroscope.tar.gz"

# TODO: verify checksum before installing, e.g.
#   echo "<expected-sha256>  $TMP/ferroscope.tar.gz" | sha256sum -c -

echo "Extracting..."
tar -xzf "$TMP/ferroscope.tar.gz" -C "$TMP"

echo "Installing binary (root:ferroscope, 0750)..."
install -o root -g ferroscope -m 0750 "$TMP/ferroscope-agent" /usr/local/bin/ferroscope-agent

# systemd part
SERVICE_NAME="ferr"
SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"

tee "$SERVICE_FILE" > /dev/null <<EOF
[Unit]
Description=ferroscope
After=network.target
StartLimitIntervalSec=600
StartLimitBurst=6

[Service]
Restart=always
RestartSec=10s

ExecStart=/usr/local/bin/ferroscope-agent
ExecReload=/bin/kill -HUP \$MAINPID

ConfigurationDirectory=ferroscope_agent
ProtectSystem=strict
ReadWritePaths=/etc/ferroscope_agent
NoNewPrivileges=true

LogsDirectory=ferroscope
StandardOutput=append:$LOG_DIR/agent.log
StandardError=append:$LOG_DIR/error.log

User=ferroscope
Group=ferroscope
[Install]
WantedBy=multi-user.target
EOF

# making the log dir
mkdir -p "$LOG_DIR"

# --- write the logrotate config --------------------------------------------
echo "Writing logrotate config to $CONF_FILE ..."
cat > "$CONF_FILE" <<EOF
$LOG_DIR/*.log {
    daily
    rotate 7
    maxsize 100M
    missingok
    notifempty
    compress
    delaycompress
    copytruncate
}
EOF

# --- correct ownership & permissions ---------------------------------------
chown root:root "$CONF_FILE"
chmod 644 "$CONF_FILE"
echo "Permissions set (root:root, 644)."

# --- validate with a dry run ------------------------------------------------
echo
echo "=== Dry run (no changes made) ==="
if logrotate -d "$CONF_FILE" 2>&1 | grep -iE "error|warning"; then
    echo "WARNING: logrotate reported issues above — review before relying on it." >&2
else
    echo "Config parsed cleanly, no errors."
fi

# --- verify the daily timer is active ---------------------------------------
echo
echo "=== logrotate timer status ==="
if systemctl list-timers logrotate.timer --no-pager 2>/dev/null | grep -q logrotate; then
    systemctl list-timers logrotate.timer --no-pager
else
    echo "WARNING: logrotate.timer not found. Rotation may rely on cron instead." >&2
    echo "Check: ls /etc/cron.daily/logrotate"
fi

# --- optional: force a rotation right now -----------------------------------
if [[ "${1:-}" == "--force" ]]; then
    echo
    echo "=== Forcing a rotation now ==="
    logrotate -f "$CONF_FILE"
    echo "Done. Current contents of $LOG_DIR:"
    ls -lh "$LOG_DIR"
fi

echo
echo "Setup complete. Logs in $LOG_DIR will rotate daily (or at 100M),"
echo "keeping 7 compressed old copies."

# restart the systemd
echo "Reloading systemd..."
systemctl daemon-reload
echo "Enabling service..."
systemctl enable "$SERVICE_NAME"
/usr/local/bin/ferroscope-agent
