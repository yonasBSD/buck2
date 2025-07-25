# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is dual-licensed under either the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree or the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree. You may select, at your option, one of the
# above-listed licenses.

load("@prelude//os_lookup:defs.bzl", "Os", "OsLookup")
load("@prelude//utils:expect.bzl", "expect")

def get_path_separator_for_exec_os(ctx: AnalysisContext) -> str:
    expect(hasattr(ctx.attrs, "_exec_os_type"), "Expect ctx.attrs._exec_os_type is defined.")
    is_windows = ctx.attrs._exec_os_type[OsLookup].os == Os("windows")
    return ";" if is_windows else ":"
