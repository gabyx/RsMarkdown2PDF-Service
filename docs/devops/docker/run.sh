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
if [ "$level" -lt 10 ]; then
    echo ">>>> $level. Container: Launching a new container ..."
    podman run -u podman --rm ttl.sh/podman-test ./run.sh "$((level + 1))"
else
    echo ">>>> $level. Container: Finally reached container level: $level"
fi

echo ">>>> $level. Container: leaving"
