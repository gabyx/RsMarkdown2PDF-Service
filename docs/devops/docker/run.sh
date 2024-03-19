#!/usr/bin/env bash
#
set -u
set -e

DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
cd "$DIR" || exit 1

level="$1"
user="${2:-root}"
vol_name="podman-root-$level"
msg="-> $level. Container"

function indent() {
    cat | sed "s@^@| @g"
}

function run_podman() {
    # When you specify `--root` , than `storage-opts`
    # in `/etc/containers/storage.conf` are ignored.
    podman \
        --root /podman-root/root \
        --runroot /podman-root/runroot \
        --storage-opt "additionalimagestore=/var/lib/shared" \
        "$@"
}

function main() {
    # Build the image.
    echo "$msg: inside [version: $(cat /image.version)]"

    # Run the image and build again.
    if [ "$level" -lt 5 ]; then
        echo "$msg: Launching a new container ..."

        # We need to make a new volume for the next podman
        # to have the stuff it needs stored.
        echo "$msg: create volume:"
        run_podman volume create "$vol_name" || {
            echo "create failed: $vol_name"
            exit 3
        }

        # We launch the new podman with root/runroot
        # on the current mounted volume `data`.
        # Then we mount the current data as
        # [`additionalimages`](https://www.redhat.com/sysadmin/image-stores-podman)
        # to next podman to have caching.

        run_podman \
            run \
            -v "$vol_name:/podman-root:Z" \
            -v "/var/lib/shared:/var/lib/shared" \
            --privileged \
            --rm ttl.sh/podman-test \
            ./run.sh "$((level + 1))" "$user" || true

        echo

    else
        echo "$msg: Finally reached container level: $level"
    fi

    echo "$msg: leaving"
}

main 2>&1 | indent
