#!/usr/bin/env bash
set -euo pipefail

# --- Config ---
BINARY_URL=""
TARGET="/usr/local/bin/ferroscope-agent"
SERVICE="ferr"

echo ">> Downloading new binary..."
curl -fSL "$BINARY_URL" -o "${TARGET}.new"

echo ">> Stopping ${SERVICE}..."
systemctl stop "$SERVICE"

echo ">> Removing old binary..."
rm -f "$TARGET"

echo ">> Putting new binary in place..."
mv "${TARGET}.new" "$TARGET"
chmod 0755 "$TARGET"

echo ">> Restarting ${SERVICE}..."
systemctl restart "$SERVICE"

echo ">> Done."