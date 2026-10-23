#!/usr/bin/env bash
# 构建 coreldraw 插件并拷贝到 cyctron/libary。
#
# 用法：
#   bash ffi/package.sh
#   CYCTRON_LIBARY=/path/to/libary bash ffi/package.sh
#
# 产物：<repo>/ffi/target/release/coreldraw.dll → <cyctron>/libary/coreldraw.dll
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LIBARY="${CYCTRON_LIBARY:-D:/codes/github/funxdata/cyctron/libary}"

cargo build --release --manifest-path "$ROOT/ffi/Cargo.toml"

SRC="$ROOT/ffi/target/release/coreldraw.dll"
DST="$LIBARY/coreldraw.dll"

if [ ! -f "$SRC" ]; then
  echo "error: build artifact not found: $SRC" >&2
  exit 1
fi

mkdir -p "$LIBARY"
cp "$SRC" "$DST"
echo "installed: $DST"
ls -l "$DST"
