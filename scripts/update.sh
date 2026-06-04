#!/usr/bin/env bash
set -euo pipefail

# --- Config ---
BINARY_URL="https://github.com/Subhodip1307/ferroscope/releases/download/v0.2.0/x86_64-unknown-linux-musl.tar.gz"
FILE="new_ferroscope.tar.gz"
BINARY_NAME="ferroscope-agent"
TARGET="/usr/local/bin/ferroscope-agent"
SERVICE="ferr"

# --- Must run as root (writes to /usr/local/bin and controls systemd) ---
if [ "$(id -u)" -ne 0 ]; then
  echo "Please run this script as root (e.g. with sudo)." >&2
  exit 1
fi

echo ">> Downloading new binary..."
curl -fL "$BINARY_URL" -o "$FILE"

echo ">> Extracting..."
tar -xzf "$FILE"

# Make sure the expected binary is actually here before we touch the service
if [ ! -f "$BINARY_NAME" ]; then
  echo "Error: '$BINARY_NAME' not found after extraction." >&2
  rm -f "$FILE"
  exit 1
fi

echo ">> Stopping ${SERVICE}..."
systemctl stop "$SERVICE"

echo ">> Replacing binary at ${TARGET}..."
rm -f "$TARGET"
mv "$BINARY_NAME" "$TARGET"
chmod +x "$TARGET"

echo ">> Starting ${SERVICE}..."
systemctl start "$SERVICE"

echo ">> Cleaning up..."
rm -f "$FILE"

echo ">> Done. ferroscope-agent updated successfully."