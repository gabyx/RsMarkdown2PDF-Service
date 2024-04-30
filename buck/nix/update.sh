#! /usr/bin/env bash
# shellcheck disable=SC1090,SC1091
set -e
set -o pipefail

ROOT_DIR=$(git rev-parse --show-toplevel)
. "$ROOT_DIR/tools/log.sh"

# ------------------------------------------------------------------------------
usage() {
    die "update.sh: perform various updates to Nix and Buck related data" \
        "Usage:" \
        "    buck run nix://:update -- [-hfba]" \
        "" \
        "  This tool is largely responsible for re-generating Buck and Nix related data" \
        "  in an automated way so things are easy to keep up to date." \
        "  Currently you can pass the following flags, to perform" \
        "  various combinations of" \
        "  the following steps, in the following given order:" \
        "" \
        "    --flake|-f         Step 1: Update the flake.lock file." \
        "    --buck2|-b         Step 2: Update buck2 nix expression" \
        "  Or, to do everything at once:" \
        "" \
        "    --all|-a           Run all steps in the above order"
}

# ------------------------------------------------------------------------------

FLAKE=0
BUCK2=0

PARSED_ARGUMENTS=$(getopt -an update.sh -o hfba --long help,flake,buck2,all -- "$@")
VALID_ARGUMENTS=$?
[ "$VALID_ARGUMENTS" != "0" ] && usage

eval set -- "$PARSED_ARGUMENTS"
while :; do
    case "$1" in
    -h | --help) usage ;;
    -f | --flake)
        FLAKE=1
        shift
        ;;
    -b | --buck2)
        BUCK2=1
        shift
        ;;
    -a | --all)
        FLAKE=1
        BUCK2=1
        shift
        ;;

    --)
        shift
        break
        ;;
    *) echo "Unexpected option: $1 - this should not happen." && usage ;;
    esac
done

[ "$FLAKE" = "0" ] && [ "$BUCK2" = "0" ] && usage

print_info "Updating flake=$FLAKE, buck2=$BUCK2"
# ------------------------------------------------------------------------------

## Step 1: Update the flake.lock file.
if [ "$FLAKE" = "1" ]; then
    nix flake --accept-flake-config update "${ROOT_DIR}/buck/nix"
fi

# ------------------------------------------------------------------------------

## Step 2: Update buck2 nix expression
if [ "$BUCK2" = "1" ]; then
    update_buck2
fi

function update_buck2() {
    local t d r v i h p

    # update the hash, revision, and version
    print_info "BUCK2: generating new version information"

    r=$(curl -sq https://api.github.com/repos/facebook/buck2/commits/main | jq -r '.sha')
    v=unstable-$(date +"%Y-%m-%d")
    i=$(nix run nixpkgs#nix-prefetch-git -- --quiet --url https://github.com/facebook/buck2 --rev "$r")
    h=$(echo "$i" | jq -r '.sha256' | xargs nix hash to-sri --type sha256)
    p=$(echo "$i" | jq -r '.path')

    sed -i 's#rev\s*=\s*".*";#rev = "'"$r"'";#' "$ROOT_DIR/buck/nix/buck2/default.nix"
    sed -i 's#hash\s*=\s*".*";#hash = "'"$h"'";#' "$ROOT_DIR/buck/nix/buck2/default.nix"
    sed -i 's#version\s*=\s*".*";#version = "'"$v"'";#' "$ROOT_DIR/buck/nix/buck2/default.nix"

    # upstream doesn't have their own Cargo.lock file, so we need to generate one
    t=$(mktemp -d)
    d="$t/buck2"

    echo "BUCK2: generating new Cargo.lock file"
    cp -r "$p" "$d" && chmod -R +w "$d"
    (cd "$d" && nix run nixpkgs#cargo -- --quiet generate-lockfile)
    cp "$d/Cargo.lock" "$ROOT_DIR/buck/nix/buck2/Cargo.lock"

    # update the toolchain based on the rust-toolchain file
    print_info "BUCK2: updating rust-toolchain setting"
    channel=$(grep -oP 'channel = \"\K\w.+(?=\")' "$p/rust-toolchain")
    if [[ $channel == nightly-* ]]; then
        # shellcheck disable=SC2001
        version=$(echo "$channel" | sed 's/nightly-//')
        sed -i 's/rustChannel\s*=\s*".*";/rustChannel = "nightly";/' "$ROOT_DIR/buck/nix/buck2/default.nix"
        sed -i 's/rustVersion\s*=\s*".*";/rustVersion = "'"$version"'";/' "$ROOT_DIR/buck/nix/buck2/default.nix"
    else
        die "Unknown channel: $channel"
    fi

    # done
    print_info "BUCK2: done\n\n"
    rm -r -f "$t"
}

# ------------------------------------------------------------------------------

## Step 3: Rebuild and push the cache
# This got deleted, because we do not use it yet.

# ------------------------------------------------------------------------------
