#!/bin/sh
# Empaqueta el binario release de Cristal en un .app y un .dmg arm64.
# No firma con un certificado de Apple. No notariza.
set -eu

root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
cd "$root/src-tauri"

version=$(awk -F '"' '/^version = / { print $2; exit }' Cargo.toml)
binary="target/release/cristal"
icon="icons/icon.icns"
stage=$(mktemp -d "${TMPDIR:-/tmp}/cristal-dmg.XXXXXX")
app="$stage/Cristal.app"

cleanup() {
  rm -rf "$stage"
}
trap cleanup EXIT

if [ ! -x "$binary" ]; then
  echo "falta $binary; ejecuta cargo build --release" >&2
  exit 1
fi
if [ ! -f "$icon" ]; then
  echo "falta $icon" >&2
  exit 1
fi

file "$binary" | grep -q "arm64" || {
  echo "el binario no es arm64" >&2
  exit 1
}

mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"
cp "$binary" "$app/Contents/MacOS/cristal"
cp "$icon" "$app/Contents/Resources/AppIcon.icns"
chmod 755 "$app/Contents/MacOS/cristal"

cat > "$app/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>es</string>
  <key>CFBundleDisplayName</key>
  <string>Cristal</string>
  <key>CFBundleExecutable</key>
  <string>cristal</string>
  <key>CFBundleIconFile</key>
  <string>AppIcon</string>
  <key>CFBundleIdentifier</key>
  <string>com.limpiarmymac.cristal</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>Cristal</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>${version}</string>
  <key>CFBundleVersion</key>
  <string>${version}</string>
  <key>LSApplicationCategoryType</key>
  <string>public.app-category.utilities</string>
  <key>NSHighResolutionCapable</key>
  <true/>
  <key>NSPrincipalClass</key>
  <string>NSApplication</string>
</dict>
</plist>
EOF
printf 'APPL????' > "$app/Contents/PkgInfo"

codesign -s - --force --deep "$app"

ln -s /Applications "$stage/Applications"

out="$root/Cristal_${version}_aarch64.dmg"
rm -f "$out"
hdiutil create -volname "Cristal ${version}" -srcfolder "$stage" -ov -format UDZO "$out" >/dev/null

sums="$root/Cristal_${version}_SHA256SUMS.txt"
shasum -a 256 "$out" | awk '{ print $1 "  " FILENAME }' FILENAME="$(basename "$out")" > "$sums"
echo "dmg=$out"
echo "sums=$sums"
shasum -a 256 "$out"
