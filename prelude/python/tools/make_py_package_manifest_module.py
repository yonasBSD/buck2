#!/usr/bin/env python3
# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is dual-licensed under either the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree or the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree. You may select, at your option, one of the
# above-listed licenses.

# pyre-strict

"""
Generate a __manifest__.py module containing build metadata for a Python package.
"""

import argparse
import json
from pathlib import Path
from typing import Dict, Optional


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=__file__.__doc__,
        fromfile_prefix_chars="@",
    )
    parser.add_argument(
        "--module-manifests",
        help="A path to a list of JSON file with modules contained in the PEX.",
        type=Path,
        default=None,
    )
    parser.add_argument(
        "--manifest-entries",
        help="Path to a JSON file with build metadata entries.",
        type=Path,
        default=None,
    )
    parser.add_argument(
        "--output",
        type=Path,
        help="Output path for the generated module.",
        required=True,
    )
    return parser.parse_args()


def path_to_module(path: str) -> Optional[str]:
    for suffix in (".py", ".so", ".pyd"):
        if path.endswith(suffix):
            return path[: -len(suffix)].replace("/", ".").replace("\\", ".")


def main() -> None:
    args = parse_args()
    output: Path = args.output
    if output.exists():
        raise ValueError(
            f"Output path '{output}' already exists, refusing to overwrite."
        )

    modules: Dict[str, str] = {}

    with open(args.module_manifests) as me:
        module_manifests = me.read().splitlines()
        for module_manifest_file in module_manifests:
            with open(module_manifest_file) as f:
                for pkg_path, _, origin_desc in json.load(f):
                    module = path_to_module(pkg_path)
                    if module:
                        modules[module] = origin_desc
                    # Add artificial __init__.py files like in make_py_package_modules.py
                    for parent in Path(pkg_path).parents:
                        if parent == Path("") or parent == Path("."):
                            continue
                        path = str(parent / "__init__.py")
                        parent_module = path_to_module(path)
                        if parent_module and parent_module not in modules:
                            modules[parent_module] = origin_desc
                        elif parent_module != module:
                            break

    entries = {}
    if args.manifest_entries:
        with open(args.manifest_entries) as f:
            entries = json.load(f)
    if not isinstance(entries, dict):
        raise ValueError(
            f"Manifest entries in {args.manifest_entries} aren't a dictionary"
        )
    if "modules" in entries:
        raise ValueError("'modules' can't be a key in manifest entries")
    sorted_modules = sorted(modules.items())
    entries["modules"] = [m[0] for m in sorted_modules]
    entries["origins"] = tuple(m[1] for m in sorted_modules)
    output.write_text(
        "\n".join((f"{key} = {repr(value)}" for key, value in entries.items()))
    )


if __name__ == "__main__":
    main()
