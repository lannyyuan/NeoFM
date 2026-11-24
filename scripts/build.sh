#!/bin/sh
# Native build on macOS or Linux. No installation or system configuration changes.
set -eu
root="$(CDPATH= cd -- "$(dirname "$0")/.." && pwd -P)"
cd "$root"
case "$(uname -s)" in
 Darwin) platform=macos;;
 Linux) platform=linux;;
 *) echo "Use scripts/build.ps1 on Windows." >&2; exit 2;;
esac
case "$(uname -m)" in
 arm64|aarch64) arch=arm64;;
 x86_64|amd64) arch=x64;;
 *) echo "Unsupported architecture: $(uname -m)" >&2; exit 2;;
esac
version="$(cat VERSION)"
out="$root/dist/neofm-$platform-$arch"
target="$root/target"
mkdir -p "$out/bin" "$out/share"
(cd "$root/lf-upstream" && CGO_ENABLED=0 go build -trimpath -buildvcs=false -ldflags='-s -w' -o "$out/bin/lf" .)
CARGO_TARGET_DIR="$target" cargo build --release --locked --manifest-path "$root/lf-integration-poc/adapter/Cargo.toml"
cp "$target/release/lfpreview" "$out/bin/"
cp "$root/platform/$platform/neofm" "$root/platform/unix/lf-trash" "$out/bin/"
chmod +x "$out/bin/neofm" "$out/bin/lf-trash"
cp -R "$root"/lf-integration-poc/fixtures-* "$out/share/"
cp "$root/LICENSE" "$out/share/LICENSE-Codex"
cp "$root/lf-upstream/LICENSE" "$out/share/LICENSE-lf"
cp "$root/NOTICE" "$root/VERSION" "$out/share/"
printf 'NeoFM v%s: %s\n' "$version" "$out"
"$out/bin/neofm" -version
