#!/usr/bin/env bash
set -e
set -u

ROOT_DIR=$(git rev-parse --show-toplevel)

cd "$ROOT_DIR"

readarray -t components < <(find ./components -mindepth 1 -maxdepth 1 -type d)
for dir in "${components[@]}"; do
    (cd "$dir" && just format)
done

git hooks exec --containerized \
    ns:githooks-shell/scripts/format-shell-all.yaml -- --force --dir "."

git hooks exec --containerized \
    ns:githooks-configs/scripts/format-configs-all.yaml -- --force --dir "."

git hooks exec --containerized \
    ns:githooks-docs/scripts/format-docs-all.yaml -- --force --dir "."

git hooks exec --containerized \
    ns:githooks-python/scripts/format-python-all.yaml -- --force --dir "."
