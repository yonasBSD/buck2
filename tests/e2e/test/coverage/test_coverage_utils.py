# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is dual-licensed under either the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree or the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree. You may select, at your option, one of the
# above-listed licenses.

# pyre-strict


import json
from pathlib import Path
from typing import List, Optional

from buck2.tests.e2e_util.api.buck import Buck


async def collect_coverage_for(
    buck: Buck,
    tmp_path: Path,
    target: str,
    folder_filter: List[str],
    file_filter: List[str],
    mode: Optional[str] = None,
    extra_tpx_args: Optional[List[str]] = None,
    extra_buck_args: Optional[List[str]] = None,
) -> List[str]:
    coverage_file = tmp_path / "coverage.txt"
    folder_filter_str = ":".join(folder_filter)
    file_filter_str = ":".join(file_filter)
    buck_args = []
    if mode is not None:
        buck_args.append(mode)
    buck_args.extend(
        [
            "--config",
            "code_coverage.enable=filtered",
            "--config",
            f"code_coverage.folder_path_filter={folder_filter_str}",
            "--config",
            f"code_coverage.file_path_filter={file_filter_str}",
        ]
        + (extra_buck_args or [])
    )
    buck_args.append(target)
    buck_args.extend(
        [
            "--",
            "--collect-coverage",
            f"--coverage-output={coverage_file}",
        ]
        + (extra_tpx_args or [])
    )
    await buck.test(*buck_args)
    paths = []
    with open(coverage_file) as results:
        for line in results:
            paths.append(json.loads(line)["filepath"])

    return list(set(paths))
