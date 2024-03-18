#!/usr/bin/env bash
#
set -u
set -e

DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
cd "$DIR" || exit 1

level="${1:-1}"
msg=">>>> $level. Container"

# Build the image.
echo "$msg: inside [version: $(cat /image.version)]"

podman=(podman
    --root /podman-root/root
    --runroot /podman-root/runroot
    --storage-opt "additionalimagestore=/var/lib/shared")

# Run the image and build again.
if [ "$level" -lt 10 ]; then
    echo "$msg: Launching a new container ..."

    # "${podman[@]}" info

    echo "$msg: podman images:"
    "${podman[@]}" images

    echo "$msg: /podman-root & /var/lib/shared permissions"
    ls -ad /podman-root /var/lib/shared

    # echo "$msg: pulling image"
    # "${podman[@]}" \
    #     pull \
    #     ttl.sh/podman-test

    echo "$msg: podman images:"
    "${podman[@]}" images
    podman images
    # sh

    # We need to make a new volume for the next podman
    # to have the stuff it needs stored.
    echo "$msg: create volume:"
    vol_name="podman-root-$level"
    "${podman[@]}" volume create "$vol_name" || {
        echo "create failed: $vol_name"
        exit 3
    }

    # We launch the new podman with root/runroot
    # on the current mounted volume `data`.
    # Then we mount the current data as
    # [`additionalimages`](https://www.redhat.com/sysadmin/image-stores-podman)
    # to next podman to have caching.
    echo "$msg: run next podman"
    "${podman[@]}" \
        run \
        -it \
        \
        -v "$vol_name:/podman-root:Z" \
        -v "/var/lib/shared:/var/lib/shared" \
        --privileged \
        --rm ttl.sh/podman-test \
        ./run.sh "$((level + 1))" || true

else
    echo "$msg: Finally reached container level: $level"
fi
echo "$msg: leaving"
