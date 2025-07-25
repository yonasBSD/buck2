# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is dual-licensed under either the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree or the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree. You may select, at your option, one of the
# above-listed licenses.

#####################################################################
# Providers

load("@prelude//utils:arglike.bzl", "ArgLike")

SwiftObjectFormat = enum(
    "object",
    "bc",
    "ir",
    "irgen",
    "object-embed-bitcode",
)

SwiftToolchainInfo = provider(
    fields = {
        "architecture": provider_field(str),
        "compiler": provider_field(cmd_args),
        "compiler_flags": provider_field(list[ArgLike]),
        "library_interface_uses_swiftinterface": provider_field(bool),
        "mk_swift_comp_db": provider_field(RunInfo),
        "mk_swift_interface": provider_field(cmd_args),
        "object_format": provider_field(SwiftObjectFormat),
        "platform_path": provider_field([Artifact, str, None]),
        "provide_swift_debug_info": provider_field(bool, default = True),
        "resource_dir": provider_field([Artifact, None]),
        "sdk_module_path_prefixes": provider_field(dict[str, Artifact]),
        "sdk_path": provider_field([Artifact, str, None]),
        "serialized_diags_to_json": provider_field([RunInfo, None], default = None),
        "supports_explicit_module_debug_serialization": provider_field(bool, default = False),
        "supports_incremental_file_hashing": provider_field(bool, default = False),
        "supports_relative_resource_dir": provider_field(bool),
        "swift_experimental_features": provider_field(dict[str, list[str]]),  # { "5": [], "6", [] }
        "swift_ide_test_tool": provider_field([RunInfo, None], default = None),
        "swift_stdlib_tool": provider_field(RunInfo),
        "swift_stdlib_tool_flags": provider_field(list[ArgLike]),
        "swift_upcoming_features": provider_field(dict[str, list[str]]),  # { "5": [], "6", [] }
        "uncompiled_clang_sdk_modules_deps": provider_field(dict[str, Dependency]),
        "uncompiled_swift_sdk_modules_deps": provider_field(dict[str, Dependency]),
        "use_depsfiles": provider_field(bool, default = False),
    },
)

# A provider that represents a non-yet-compiled SDK (Swift or Clang) module,
# and doesn't contain any artifacts because Swift toolchain isn't resolved yet.
SdkUncompiledModuleInfo = provider(fields = {
    # [Dependency]
    "cxx_deps": provider_field(typing.Any, default = None),
    # [Dependency]
    "deps": provider_field(typing.Any, default = None),
    # A relative prefixed path to a textual swiftinterface/modulemap file within an SDK.
    "input_relative_path": provider_field(typing.Any, default = None),
    # This is mostly needed for the generated Swift module map file.
    "is_framework": provider_field(typing.Any, default = None),
    # If True then represents a swiftinterface, otherwise Clang's modulemap.
    "is_swiftmodule": provider_field(typing.Any, default = None),
    # A real name of a module, without distinguishing suffixes.
    "module_name": provider_field(typing.Any, default = None),
    # Partial arguments, required to compile a particular SDK module.
    "partial_cmd": provider_field(typing.Any, default = None),
    # A string of the compiler target triple to use for clang module deps, eg arm64-apple-ios16.4
    "target": provider_field(typing.Any, default = None),
})

WrappedSdkCompiledModuleInfo = provider(fields = {
    # A tset of PCM artifacts
    "clang_debug_info": provider_field(typing.Any, default = None),
    # A SwiftCompiledModuleTset of SwiftCompiledModuleInfo of transitive clang deps
    "clang_deps": provider_field(typing.Any, default = None),
    # A tset of swiftmodule artifacts
    "swift_debug_info": provider_field(typing.Any, default = None),
    # A SwiftCompiledModuleTset of SwiftCompiledModuleInfo of transitive swift deps
    "swift_deps": provider_field(typing.Any, default = None),
})

SdkSwiftOverlayInfo = provider(fields = {
    "overlays": provider_field(typing.Any, default = None),  # {str: [str]} A mapping providing a list of overlay module names for each underlying module
})

SwiftCompiledModuleInfo = provider(fields = {
    # Additional flags for the clang importer.
    "clang_importer_args": provider_field(cmd_args | None, default = None),
    # Include flags for the clang importer.
    "clang_module_file_args": provider_field(cmd_args | None, default = None),
    # Clang modulemap as args which is required for generation of swift_module_map.
    "clang_modulemap_args": provider_field(cmd_args | None, default = None),
    # Clang modulemap artifact and associated artifacts (e.g., symlink dir
    # with headers). For SDK modules, the associated artifact would be the
    # SDK directory itself (which contains modulemaps).
    "clang_modulemap_artifacts": provider_field(list[Artifact], default = []),
    # If present an artifact for the modules swiftinterface.
    "interface_artifact": provider_field(Artifact | None, default = None),
    "is_framework": provider_field(bool),
    "is_sdk_module": provider_field(bool),
    # If True then contains a compiled swiftmodule, otherwise Clang's pcm.
    "is_swiftmodule": provider_field(bool),
    # A real name of a module, without distinguishing suffixes.
    "module_name": provider_field(str),
    # Compiled artifact either swiftmodule or pcm.
    "output_artifact": provider_field(Artifact),
})

def _add_swiftmodule_search_path(module_info: SwiftCompiledModuleInfo):
    # We need to import the containing folder, not the file itself.
    # We skip SDK modules as those are found via the -sdk flag.
    if module_info.is_swiftmodule and not module_info.is_sdk_module:
        return ["-I", cmd_args(module_info.output_artifact, parent = 1)]

    return []

def _add_clang_module_file_flags(module_info: SwiftCompiledModuleInfo):
    if module_info.is_swiftmodule:
        return []
    else:
        return [module_info.clang_module_file_args]

def _add_clang_importer_flags(module_info: SwiftCompiledModuleInfo):
    if module_info.is_swiftmodule:
        return []
    else:
        return [module_info.clang_importer_args] if module_info.clang_importer_args else []

def _swift_module_map_struct(module_info: SwiftCompiledModuleInfo):
    if module_info.is_swiftmodule:
        # Swiftmodule files compiled from swiftinterface files embed the paths
        # which are verified during compilation of rdeps, so we need to add
        # the swiftinterface files as hidden inputs.
        module_path = cmd_args(
            module_info.output_artifact,
            hidden = filter(None, [module_info.interface_artifact]),
            delimiter = "",
        )

        return struct(
            isFramework = module_info.is_framework,
            moduleName = module_info.module_name,
            modulePath = module_path,
        )
    else:
        return struct(
            isFramework = module_info.is_framework,
            moduleName = module_info.module_name,
            clangModulePath = module_info.output_artifact,
            clangModuleMapPath = cmd_args([module_info.clang_modulemap_args], delimiter = ""),
        )

SwiftCompiledModuleTset = transitive_set(
    args_projections = {
        "clang_importer_flags": _add_clang_importer_flags,  # Additional clang flags required for compilation.
        "clang_module_file_flags": _add_clang_module_file_flags,  # Projects pcm modules as cli flags.
        "module_search_path": _add_swiftmodule_search_path,
    },
    json_projections = {
        "swift_module_map": _swift_module_map_struct,
    },
)
