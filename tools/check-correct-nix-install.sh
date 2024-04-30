#!/usr/bin/env bash
# shellcheck disable=SC1090,SC1091
# Source: https://github.com/thoughtpolice/buck2-nix/blob/main/.envrc

set -e
set -u

ROOT_DIR=$(git rev-parse --show-toplevel)
. "$ROOT_DIR/tools/log.sh"

# First, check for a supported nix version
MSNV="2.14.0"
OURNV=$(nix eval --raw --expr "builtins.nixVersion")
vercomp=$(nix eval --expr "builtins.compareVersions \"${OURNV}\" \"${MSNV}\"")
if [ "$vercomp" = "-1" ]; then

    print_warning \
        "ERROR: Your Nix is older than the "Minimum Supported Nix Version" (MSNV)." \
        " Your Nix: ${OURNV}" \
        "     MSNV: ${MSNV}" \
        "This build can't be guaranteed to succeed correctly or be free of bugs that" \
        "the system may otherwise guarantee." \
        "If you can't freely upgrade Nix, then please open a discussion if you would" \
        "like the minimum supported Nix version to be adjusted."

    [ -z "$NIX_ALLOW_UNSUPPORTED_VERSION" ] && exit 1
    printf "NOTICE: Continuing despite unsupported Nix version.\n\n"
fi

# Next, check that the user is trusted, so the out-of-band binary cache
# can be used (this is required even with ca-derivations)
if ! nix show-config | grep -q "trusted-users =.*${USER}"; then
    die \
        "ERROR: Your user account must be part of the 'trusted-users' setting in your" \
        "Nix configuration to use this project. Please add '$USER' to the" \
        "'trusted-users' setting in either:" \
        " - your /etc/nixos/configuration.nix" \
        " - your /etc/nix/nix.conf"
fi

# Make sure the user has [ref:ca-derivations] enabled; otherwise, the ability to fetch
# derivations via Nix won't work due to the lack of self-authentication.
if ! nix show-config | grep -q 'experimental-features.*ca-derivations'; then
    die \
        "ERROR: You must enable the 'ca-derivations' in your Nix configuration to use" \
        "this project. See https://nixos.wiki/wiki/Ca-derivations for more information" \
        "and modify either: " \
        " - your /etc/nixos/configuration.nix'" \
        " - your /etc/nix/nix.conf'"
fi
