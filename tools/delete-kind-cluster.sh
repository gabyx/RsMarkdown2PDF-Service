#!/usr/bin/env bash
# Check the script in https://kind.sigs.k8s.io/docs/user/local-registry/
# for further information.
set -e
set -u

cluster_name="$1"
reg_name="kind-registry"

# create registry container unless it already exists
running="$(docker inspect -f '{{.State.Running}}' "${reg_name}" 2>/dev/null || true)"
if [ "${running}" == 'true' ]; then
    cid="$(docker inspect -f '{{.ID}}' "${reg_name}")"

    echo "Stopping and deleting Kind Registry container..."
    docker stop "$cid" >/dev/null
    docker rm "$cid" >/dev/null
fi

echo "Deleting Kind cluster '$cluster_name'..."
kind delete cluster --name="$cluster_name"
