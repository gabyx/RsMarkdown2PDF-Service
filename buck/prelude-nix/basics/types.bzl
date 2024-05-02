# SPDX-FileCopyrightText: © 2022 Austin Seipp
# SPDX-License-Identifier: Apache-2.0

# @prelude-nix//basics/types.bzl -- Global types for Buck rules.
#
# HOW TO USE THIS MODULE:
#
#   N/A. Providers are automatically loaded by the prelude.

"""Providers used by all Buck rules in this prelude."""

load("@prelude-nix//basics/files.bzl", "files")
load("@prelude-nix//toolchains/nixpkgs.bzl", "nix");
load("@prelude-nix//toolchains/bash/main.bzl", "bash");
load("@prelude-nix//toolchains/rust/main.bzl", "rust");

## ---------------------------------------------------------------------------------------------------------------------

# Global attributes. Key->value pairs of attribute names and their types.
attributes = { }

# Global providers. Key->value pairs of provider names and their types.
providers: dict[str, Provider] = { }

## ---------------------------------------------------------------------------------------------------------------------

_ALL_MODULES = [
    files,
    nix,
    bash,
    rust,
]

## ---------------------------------------------------------------------------------------------------------------------

for mod in _ALL_MODULES:
    provs = getattr(mod, "providers")
    for (k, v) in provs.items():
        if k in providers:
            fail("Provider '{}' already exists!".format(k))
        providers[k] = v

    attrs = getattr(mod, "attributes")
    for (k, v) in attrs.items():
        if k in attributes:
            fail("Attribute '{}' already exists!".format(k))
        attributes[k] = v
