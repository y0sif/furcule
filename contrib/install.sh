#!/usr/bin/env sh
# Furcule installer: downloads the latest release binary for this platform.
# Usage: curl -fsSL https://raw.githubusercontent.com/y0sif/furcule/main/contrib/install.sh | sh
set -eu

REPO="y0sif/furcule"
BIN="furcule"
INSTALL_DIR="${FURCULE_INSTALL_DIR:-$HOME/.local/bin}"

say()  { printf '\033[1;36m==>\033[0m %s\n' "$1"; }
fail() { printf '\033[1;31merror:\033[0m %s\n' "$1" >&2; exit 1; }

say "1/4 Detecting platform"
os=$(uname -s); arch=$(uname -m)
if [ "$os" = "Linux" ]; then
  if [ "$arch" = "x86_64" ]; then target="x86_64-unknown-linux-gnu"; elif [ "$arch" = "aarch64" ] || [ "$arch" = "arm64" ]; then target="aarch64-unknown-linux-gnu"; else fail "unsupported Linux arch: $arch"; fi
elif [ "$os" = "Darwin" ]; then
  if [ "$arch" = "arm64" ]; then target="aarch64-apple-darwin"; elif [ "$arch" = "x86_64" ]; then target="x86_64-apple-darwin"; else fail "unsupported macOS arch: $arch"; fi
else
  fail "unsupported OS: $os. On Windows, download the zip from https://github.com/$REPO/releases"
fi
say "    $os $arch -> $target"

say "2/4 Finding latest release"
tag=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p' | head -1)
[ -n "$tag" ] || fail "could not read latest release tag"
name="$BIN-$tag-$target"
url="https://github.com/$REPO/releases/download/$tag/$name.tar.gz"
say "    $tag"

say "3/4 Downloading and verifying"
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
curl -fsSL "$url" -o "$tmp/$name.tar.gz"
curl -fsSL "$url.sha256" -o "$tmp/$name.tar.gz.sha256"
(cd "$tmp" && (shasum -a 256 -c "$name.tar.gz.sha256" >/dev/null 2>&1 || sha256sum -c "$name.tar.gz.sha256" >/dev/null)) || fail "checksum mismatch"
tar -xzf "$tmp/$name.tar.gz" -C "$tmp"

say "4/4 Installing to $INSTALL_DIR"
mkdir -p "$INSTALL_DIR"
install -m 755 "$tmp/$name/$BIN" "$INSTALL_DIR/$BIN"
case ":$PATH:" in *":$INSTALL_DIR:"*) ;; *) say "    add $INSTALL_DIR to your PATH";; esac
say "done: $("$INSTALL_DIR/$BIN" --version)"
