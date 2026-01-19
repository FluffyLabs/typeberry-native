#!/bin/bash
set -euo pipefail
set -x

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$SCRIPT_DIR"

cd "$ROOT"

TARGET="${TARGET:-${1:-}}"
if [ -n "$TARGET" ]; then
  cargo build --release --target "$TARGET" -p bandersnatch-native
  target_dir="$ROOT/target/$TARGET"
else
  cargo build --release -p bandersnatch-native
  target_dir="$ROOT/target"
  TARGET="$(rustc -vV | sed -n 's/^host: //p')"
fi

case "$TARGET" in
  aarch64-apple-darwin)
    package="darwin-arm64"
    lib_ext="dylib"
    ;;
  x86_64-unknown-linux-gnu)
    package="linux-x64-gnu"
    lib_ext="so"
    ;;
  *)
    echo "Unsupported target: $TARGET"
    exit 1
    ;;
esac

artifact="$target_dir/release/libbandersnatch_native.$lib_ext"
dest_dir="$ROOT/npm/$package"

mkdir -p "$dest_dir"
cp "$artifact" "$dest_dir/bandersnatch.$package.node"
