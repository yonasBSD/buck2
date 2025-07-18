# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is dual-licensed under either the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree or the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree. You may select, at your option, one of the
# above-listed licenses.

load("@fbcode_macros//build_defs:rust_binary.bzl", "rust_binary")
load("@fbcode_macros//build_defs/lib:link_styles.bzl", "link_styles")

def buck_rust_binary(**kwargs):
    kwargs.setdefault("edition", "2021")
    if kwargs.get("link_style") == None:
        link_style = link_styles.default()
        kwargs["link_style"] = select({
            "DEFAULT": link_style,
            "ovr_config//os:macos": "static",
        })

    # Link group is currently used automatically for rust in dev mode. Unfortunately, it builds
    # a binary that is not relocatable and it checks for dev mode by reading the build mode buckconfig.
    # If we don't disable link groups, we will also end up building a non-relocatable binary when
    # using opt modifier because opt modifier does not change build mode buckconfig. Work around this
    # by disabling link groups for now.
    # TODO(scottcao): Delete this line once link group macros are properly selectified.
    kwargs["link_group_map"] = []

    # JEMalloc is not (yet!) the default on MacOS so add the allocator
    # explicitly on all platforms here.
    kwargs.setdefault("allocator", "jemalloc")
    rust_binary(**kwargs)
