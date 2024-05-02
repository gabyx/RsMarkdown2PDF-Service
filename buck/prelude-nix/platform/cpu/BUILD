# SPDX-FileCopyrightText: © 2022 Meta Platforms, Inc. and affiliates.
# SPDX-FileCopyrightText: © 2022 Austin Seipp
# SPDX-License-Identifier: MIT OR Apache-2.0

load("@prelude-nix//basics/config.bzl", "config")

config.setting(
    name = "x86_64",
    constraints = [ "prelude-nix//platform/cpu/constraints:x86_64" ],
    visibility = [ "PUBLIC" ],
)

config.setting(
    name = "aarch64",
    constraints = [ "prelude-nix//platform/cpu/constraints:aarch64" ],
    visibility = [ "PUBLIC" ],
)
