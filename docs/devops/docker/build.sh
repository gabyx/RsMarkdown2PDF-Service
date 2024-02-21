#!/usr/bin/env bash
#
set -u
# set -e

DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
cd "$DIR" || exit 1

level="${1:-1}"

# Build the image.
echo ">>>> $level. Container: inside"

# Run the image and build again.
if [ "$level" -lt 3 ]; then
    echo ">>>> $level. Container: Building podman..."
    podman build -t podman-test . 1>/dev/null

    echo ">>>> $level. Container: Launching a new container ..."
    podman run -u podman --rm podman-test ./build.sh "$((level + 1))"
else
    echo ">>>> $level. Container: Finally reached container level: $level"
fi

echo ">>>> $level. Container: leaving"
