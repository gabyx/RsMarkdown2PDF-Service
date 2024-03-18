#!/usr/bin/env bash
#
set -u
set -e

DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
cd "$DIR" || exit 1

level="${1:-1}"
msg=">>>> $level. Container"

# Build the image.
echo "$msg: inside [version: $(cat ~/image.version)]"

# Run the image and build again.
if [ "$level" -lt 10 ]; then
    echo "$msg: Launching a new container ..."

    # We need to make a new volume for the next podman
    # to have the stuff it needs stored.

    echo "$msg: create volume:"
    storage_name="storage"
    podman volume create "$storage_name" || {
        echo "create failed: $storage_name"
        exit 3
    }

    echo "$msg: volumes:"
    podman volume list || {
        echo "list failed"
        exit 1
    }

    # We launch the new podman with root/runroot
    # on the current mounted volume `storage`.
    # Then we mount the current storage as
    # [`additionalimages`](https://www.redhat.com/sysadmin/image-stores-podman)
    # to next podman to have caching.

    podman \
        --root "/storage/root" \
        --runroot "/storage/runroot" \
        run \
        --privileged \
        -v "$storage_name:/storage" \
        -v "/storage:/var/lib/shared" \
        --rm ttl.sh/podman-test \
        ./run.sh "$((level + 1))" || true

else
    echo "$msg: Finally reached container level: $level"
fi
echo "$msg: leaving"
