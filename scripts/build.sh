#!/usr/bin/env bash
set -e

# Check cross
export PATH="$HOME/.cargo/bin:$PATH"
if ! command -v cross &> /dev/null
then
    echo "cross not found, installing..."
    cargo install cross --force
fi
export PATH="$HOME/.cargo/bin:$PATH"

# Output directory
OUTPUT_DIR="binaries"
mkdir -p "$OUTPUT_DIR"

# Targets list
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "armv7-unknown-linux-gnueabihf"
    "aarch64-unknown-linux-gnu"
    "x86_64-unknown-freebsd"
    "x86_64-pc-windows-gnu"
    "i686-pc-windows-gnu"
)

# Build function
build_target() {
    local target=$1
    echo "Building $target ..."
    cross build --release --target "$target"

    # Binary name
    BIN_NAME="tick"

    # Destination folder by platform
    DEST_DIR="$OUTPUT_DIR/$target"
    mkdir -p "$DEST_DIR"

    # Extension for Windows binaries
    EXT=""
    if [[ "$target" == *"windows"* ]]; then
        EXT=".exe"
    fi

    # Copy binary
    cp "../target/$target/release/$BIN_NAME$EXT" "$DEST_DIR/$BIN_NAME-$target$EXT"
    echo "Built $DEST_DIR/$BIN_NAME-$target$EXT"
}

# Loop through targets
for t in "${TARGETS[@]}"; do
    build_target "$t"
done

echo "All done!"
