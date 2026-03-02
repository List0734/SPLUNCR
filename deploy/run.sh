#!/bin/sh
set -e
SCRIPT_DIR="$(realpath "$(dirname "$0")")"

"$SCRIPT_DIR/deploy.sh" &
cargo run -p station --release
