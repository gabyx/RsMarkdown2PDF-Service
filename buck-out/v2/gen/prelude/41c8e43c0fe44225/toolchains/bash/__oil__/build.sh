#!/usr/bin/env bash
set -euo pipefail
[ -f /buildbarn/profile ] && source /buildbarn/profile
export NIX_PATH=
nix build \
  -I buckroot=$PWD \
  -I buckpkgs=file://$PWD/buck-out/v2/gen/prelude/41c8e43c0fe44225/toolchains/__nixpkgs.tar.gz__/nixpkgs.tar.gz \
  -I overlay-rust=file://$PWD/buck-out/v2/gen/prelude/41c8e43c0fe44225/toolchains/__nixpkgs-overlay-rust.tar.gz__/nixpkgs-overlay-rust.tar.gz \
  -f buck-out/v2/gen/prelude/41c8e43c0fe44225/toolchains/bash/__oil__/build.nix \
  --out-link "$1"
