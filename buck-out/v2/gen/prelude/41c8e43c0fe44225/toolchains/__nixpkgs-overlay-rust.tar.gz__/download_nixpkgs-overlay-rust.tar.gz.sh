#!/usr/bin/env bash
set -xeuo pipefail
[ -f /buildbarn/profile ] && source /buildbarn/profile
curl -Lo "$1" https://github.com/oxalica/rust-overlay/archive/ffe47b90076067ad5dc25fe739d95a463bdf3c59.tar.gz
hash=$(nix hash path --type sha256 "$1")
if ! [ "$hash" = "sha256-XU8xT/hQrg5R1MgUPXOKeqBxiiOYTki758cQc4U8OUs=" ]; then
  echo "hash mismatch:"
  echo "  expected 'sha256-XU8xT/hQrg5R1MgUPXOKeqBxiiOYTki758cQc4U8OUs='"
  echo "       got '$hash'"
  exit 1
fi
