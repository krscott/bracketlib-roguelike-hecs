#!bash

# exit when any command fails
set -e
trap 'echo ''; echo Error at $(basename "$0"):${LINENO}: $BASH_COMMAND' ERR

# set working directory to this script's directory
cd "${0%/*}"

PROJECT_NAME="bracketlib-book"
OUT_DIR="dist/wasm"

mkdir -p "$OUT_DIR"

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen "target/wasm32-unknown-unknown/release/$PROJECT_NAME.wasm" --out-dir "$OUT_DIR" --no-modules --no-typescript
cp "src/web/"* "$OUT_DIR"
