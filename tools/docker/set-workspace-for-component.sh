#!/usr/bin/env bash
# Check the script in https://kind.sigs.k8s.io/docs/user/local-registry/
# for further information.
set -e
set -u

component="$1"

tomlq -t ".workspace.members |= [ \"components/$component\", \"components/common\"]" Cargo.toml >Cargo.toml.tmp
mv Cargo.toml.tmp Cargo.toml

echo "Modified Cargo workspaces:"
cat Cargo.toml
