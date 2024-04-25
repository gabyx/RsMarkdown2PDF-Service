#!/usr/bin/env bash
set -xeuo pipefail
[ -f /buildbarn/profile ] && source /buildbarn/profile
curl -Lo "$1" https://github.com/nixos/nixpkgs/archive/962d920b8dff4607dd27d33c36c88e4882f62a96.tar.gz
hash=$(nix hash path --type sha256 "$1")
if ! [ "$hash" = "sha256-VhcLZn+Y27qyJbeC73YH2c4XhIE5ipDDwtBydxjmtw8=" ]; then
  echo "hash mismatch:"
  echo "  expected 'sha256-VhcLZn+Y27qyJbeC73YH2c4XhIE5ipDDwtBydxjmtw8='"
  echo "       got '$hash'"
  exit 1
fi
