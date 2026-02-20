#!/bin/sh
set -e
SCRIPT_DIR="$(realpath "$(dirname "$0")")"

TARGET=aarch64-unknown-linux-musl
ROBOT_HOST=user@192.168.100.1
BIN=robot

cross build -p robot --release --target $TARGET

scp "$SCRIPT_DIR/../target/$TARGET/release/$BIN" "$ROBOT_HOST:/home/user/robot.new"
scp "$SCRIPT_DIR/../crates/robot/config.toml" "$ROBOT_HOST:/home/user/config.toml"

ssh -t "$ROBOT_HOST" << 'EOF' 2>&1 | grep -v "Pseudo-terminal will not be allocated"
# Stop service, ignore errors if not running
doas rc-service robot stop || true

# Replace binary
mv /home/user/robot.new /home/user/robot
chmod +x /home/user/robot

# Start service
doas rc-service robot start
EOF

cargo run -p station --release