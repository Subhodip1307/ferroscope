#!/usr/bin/env bash
set -euo pipefail

# --- Config ---
VERSION="v2.2"
BINARY_URL="https://github.com/Subhodip1307/ferroscope/releases/download/${VERSION}/x86_64-unknown-linux-musl.tar.gz"
BINARY_NAME="ferroscope-agent"
TARGET="/usr/local/bin/ferroscope-agent"
SERVICE="ferr"

# --- Must run as root (writes to /usr/local/bin and controls systemd) ---
if [ "$(id -u)" -ne 0 ]; then
  echo "Please run this script as root (e.g. with sudo)." >&2
  exit 1
fi

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

echo ">> Downloading ${VERSION}..."
curl -fL --proto '=https' "$BINARY_URL" -o "$TMP/ferroscope.tar.gz"

# TODO: verify checksum before installing, e.g.
#   echo "<expected-sha256>  $TMP/ferroscope.tar.gz" | sha256sum -c -

echo ">> Extracting..."
tar -xzf "$TMP/ferroscope.tar.gz" -C "$TMP"

if [ ! -f "$TMP/$BINARY_NAME" ]; then
  echo "Error: '$BINARY_NAME' not found in the archive." >&2
  exit 1
fi

echo ">> Installing new binary at ${TARGET}..."
install -o root -g ferroscope -m 0750 "$TMP/$BINARY_NAME" "$TARGET"

echo ">> Restarting ${SERVICE}..."
if systemctl cat "$SERVICE" > /dev/null 2>&1; then
  systemctl restart "$SERVICE"
else
  echo "Warning: service '$SERVICE' not found — binary updated, nothing restarted." >&2
fi

echo ">> Done. ferroscope-agent updated to ${VERSION}."