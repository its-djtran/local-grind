#!/usr/bin/env bash
# Downloads the relocatable CPython bundled with the desktop app (python-build-standalone,
# PSF license) into app/src-tauri/resources/python, verifying a pinned checksum.
# Usage: scripts/fetch_python.sh [aarch64|x86_64]   (defaults to this machine's architecture)
set -euo pipefail
cd "$(dirname "$0")/.."

TAG=20261003
VERSION=3.13.16
ARCH="${1:-$(uname -m)}"
[[ "$ARCH" == "arm64" ]] && ARCH=aarch64
case "$ARCH" in
  aarch64) SHA=9e01f63bbb08576cd9c8bc2d0564d098cb30c8453a0cd4bcf6aef458f6d2a147 ;;
  x86_64) SHA=b4dad38ba6a344555ccb71a1b08caad0a6c0dda88c5803658bc95bd7f04e9f5c ;;
  *) echo "unsupported architecture: $ARCH" >&2; exit 1 ;;
esac

DEST=app/src-tauri/resources/python
STAMP="$DEST/.lg-version"
WANT="$VERSION+$TAG-$ARCH"
if [[ -f "$STAMP" && "$(cat "$STAMP")" == "$WANT" ]]; then
  echo "bundled Python $WANT already present"
  exit 0
fi

FILE="cpython-$VERSION+$TAG-$ARCH-apple-darwin-install_only_stripped.tar.gz"
URL="https://github.com/astral-sh/python-build-standalone/releases/download/$TAG/${FILE//+/%2B}"
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
echo "downloading $FILE"
curl -fsSL "$URL" -o "$TMP/$FILE"
echo "$SHA  $TMP/$FILE" | shasum -a 256 -c - >/dev/null
rm -rf "$DEST"
mkdir -p "$(dirname "$DEST")"
tar -xzf "$TMP/$FILE" -C "$TMP"
mv "$TMP/python" "$DEST"
# Drop parts no judged solution can use (GUI toolkits, IDLE, pip, headers, test data).
(
  cd "$DEST"
  rm -rf include share lib/tcl* lib/tk* lib/itcl* lib/libtcl* lib/libtk* \
    lib/python3.*/{idlelib,tkinter,turtledemo,ensurepip,pydoc_data,site-packages/pip*} \
    lib/python3.*/lib-dynload/_tkinter* bin/idle* bin/pip* bin/pydoc* \
    lib/pkgconfig lib/thread* bin/*-config
  # Bundlers copy symlinks as full files, so keep one real interpreter named python3.
  rm -f bin/python bin/python3
  mv bin/python3.* bin/python3
)
echo "$WANT" > "$STAMP"
"$DEST/bin/python3" --version
du -sh "$DEST"
