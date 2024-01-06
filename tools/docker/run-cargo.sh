#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091
#
set -e
set -u

build_type="$1"

if [ "$build_type" = "debug" ]; then

    echo "WARNING: Building debug build ...!" >&2
    cargo build -p "$COMPONENT"
    echo "WARNING: Built debug configuration!" >&2

elif [ "$build_type" = "release" ]; then

    cargo build --release -p "$COMPONENT"

else
    echo "Build type not supported: '$build_type'" >&2
    exit 1
fi
