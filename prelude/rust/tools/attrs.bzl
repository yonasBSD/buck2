# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under both the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree and the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree.

def _internal_tool(default: str) -> Attr:
    return attrs.default_only(attrs.exec_dep(providers = [RunInfo], default = default))

# Factored out of prelude//toolchains/rust.bzl to keep only the user-facing
# configurable attributes there. This list of internal tools is distracting and
# expected to grow.
internal_tool_attrs = {
    "deferred_link_action": _internal_tool("prelude//rust/tools:deferred_link_action"),
    "extract_link_action": _internal_tool("prelude//rust/tools:extract_link_action"),
    "failure_filter_action": _internal_tool("prelude//rust/tools:failure_filter_action"),
    "llvm_lines_output_redirect": _internal_tool("prelude//rust/tools:llvm_lines_output_redirect"),
    "rustc_action": _internal_tool("prelude//rust/tools:rustc_action"),
    "rustdoc_coverage": _internal_tool("prelude//rust/tools:rustdoc_coverage"),
    "rustdoc_test_with_resources": _internal_tool("prelude//rust/tools:rustdoc_test_with_resources"),
    "transitive_dependency_symlinks_tool": _internal_tool("prelude//rust/tools:transitive_dependency_symlinks"),
}
