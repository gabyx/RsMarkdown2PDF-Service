#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091
#
set -e
set -u

component="$1"
target="$2"
build_type="$3"

if [ "$build_type" = "debug" ]; then

    echo "WARNING: Building debug build ...!" >&2
    cargo build -p "$component" --bin "$target"
    echo "WARNING: Built debug configuration!" >&2

elif [ "$build_type" = "release" ]; then

    cargo build --release -p "$component" --bin "$target"

else
    echo "Build type not supported: '$build_type'" >&2
    exit 1
fi
