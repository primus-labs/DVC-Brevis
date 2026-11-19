#!/usr/bin/env bash
# Regenerate the self-signed TLS cert/key used by https_server.py
# (cert.pem + key.pem are not committed; run this once after cloning).
set -euo pipefail
cd "$(dirname "$0")"

openssl req -x509 -newkey rsa:4096 -nodes -out cert.pem -keyout key.pem -days 365 -subj "/CN=localhost"

echo "Generated cert.pem, key.pem"
