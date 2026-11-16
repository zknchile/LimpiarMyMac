#!/bin/sh
# Dibuja el icono plano de Cristal y arma el .icns. No es vidrio.
set -eu

root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
stage=$(mktemp -d "${TMPDIR:-/tmp}/cristal-icon.XXXXXX")
icons="$root/src-tauri/icons"

cleanup() {
  rm -rf "$stage"
}
trap cleanup EXIT

export DEVELOPER_DIR="${DEVELOPER_DIR:-/Applications/Xcode.app/Contents/Developer}"
swift "$root/scripts/make_icon.swift" "$stage"

set_icon() {
  cp "$stage/icon-$1.png" "$stage/icon.iconset/$2"
}

mkdir -p "$stage/icon.iconset"
set_icon 16 icon_16x16.png
set_icon 32 icon_16x16@2x.png
set_icon 32 icon_32x32.png
set_icon 64 icon_32x32@2x.png
set_icon 128 icon_128x128.png
set_icon 256 icon_128x128@2x.png
set_icon 256 icon_256x256.png
set_icon 512 icon_256x256@2x.png
set_icon 512 icon_512x512.png
cp "$stage/icon.png" "$stage/icon.iconset/icon_512x512@2x.png"

iconutil -c icns "$stage/icon.iconset" -o "$icons/icon.icns"
cp "$stage/icon.png" "$icons/icon.png"
echo "png=$icons/icon.png"
echo "icns=$icons/icon.icns"
