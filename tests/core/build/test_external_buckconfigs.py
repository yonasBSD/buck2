# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under both the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree and the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree.

# pyre-strict

import json
import tempfile
from pathlib import Path

from buck2.tests.e2e_util.api.buck import Buck
from buck2.tests.e2e_util.buck_workspace import buck_test
from buck2.tests.e2e_util.helper.utils import filter_events, read_invocation_record


@buck_test(
    extra_buck_config={
        "external_path_configs_section": {
            "external_path_configs_key": "external_path_configs_value",
        }
    },
)
async def test_external_buckconfigs(buck: Buck) -> None:
    with tempfile.NamedTemporaryFile("w", delete=False) as f:
        f.write("[test_section]\n")
        f.write("test_key = test_value\n")
        f.close()
        await buck.build(
            "@root//mode/my_mode",
            "//:test",
            "-c",
            "my_section.my_key=my_value",
            "--config-file",
            f.name,
        )
        # Make a spurious file change to trigger DICE updater state comparison
        with open(buck.cwd / "src", "w") as src:
            src.write("test")

        await buck.build(
            "@root//mode/my_mode",
            "//:test",
            "-c",
            "my_section.my_key=my_value",
            "--config-file",
            f.name,
        )

    buckconfig_input_values = await filter_events(
        buck, "Event", "data", "Instant", "data", "BuckconfigInputValues", "components"
    )
    assert len(buckconfig_input_values) == 1
    external_configs = buckconfig_input_values[0]

    assert len(external_configs) == 4
    external_index = 0
    # The order is important here. We first have the buckconfig values from external sources
    external_path_configs = external_configs[0]["data"]["GlobalExternalConfigFile"]
    # Our tests inject file_watcher to external configs in test setup stage
    injected_config_value = external_path_configs["values"][external_index]
    assert (
        injected_config_value["section"] == "buck2"
        and injected_config_value["key"] == "file_watcher"
        and injected_config_value["value"] == "fs_hash_crawler"
        and not injected_config_value["is_cli"]
    )
    external_index += 1
    external_path_config_value = external_path_configs["values"][external_index]
    assert (
        external_path_config_value["section"] == "external_path_configs_section"
        and external_path_config_value["key"] == "external_path_configs_key"
        and external_path_config_value["value"] == "external_path_configs_value"
        and not external_path_config_value["is_cli"]
    )
    # since the origin is a tempfile, we can't assert the exact path
    assert external_path_configs["origin_path"].endswith("extra.bcfg")
    # The rest matches the same order provided by the above buck command
    # i.e. modefile, command line config flag, followed by the config file

    # We only store the path of the modefile
    assert (
        external_configs[1]["data"]["ConfigFile"]["data"]["ProjectRelativePath"]
        == "my_mode.bcfg"
    )

    # Then comes the config flag from the cli
    config_flag = external_configs[2]["data"]["ConfigValue"]
    assert config_flag["is_cli"]
    assert (
        config_flag["section"] == "my_section"
        and config_flag["key"] == "my_key"
        and config_flag["value"] == "my_value"
    )
    config_file = external_configs[3]["data"]["ConfigFile"]["data"][
        "GlobalExternalConfig"
    ]

    # Finally, the config file whose values are marked as coming from cli too
    config_file_value = config_file["values"][0]
    assert config_file_value["is_cli"]
    assert (
        config_file_value["section"] == "test_section"
        and config_file_value["key"] == "test_key"
        and config_file_value["value"] == "test_value"
    )
    assert (
        f.name in config_file["origin_path"]
    ), f"Origin path should contain config-file name: {f.name}"


@buck_test(
    extra_buck_config={
        "external_path_configs_section": {
            "external_path_configs_key": "external_path_configs_value",
        }
    },
)
async def test_previous_command_with_mismatched_config(
    buck: Buck, tmp_path: Path
) -> None:
    await buck.build(
        "@root//mode/my_mode",
        "//:test",
        "-c",
        "my_section.my_key=my_value",
    )
    previous_invalidating_command = await filter_events(
        buck, "Event", "data", "Instant", "data", "PreviousCommandWithMismatchedConfig"
    )
    # No previous command, no PreviousCommandWithMismatchedConfig fired
    assert len(previous_invalidating_command) == 0

    # Rerun without any changes
    res = await buck.build(
        "@root//mode/my_mode",
        "//:test",
        "-c",
        "my_section.my_key=my_value",
    )
    trace_id = json.loads(res.stdout)["trace_id"]
    previous_invalidating_command = await filter_events(
        buck, "Event", "data", "Instant", "data", "PreviousCommandWithMismatchedConfig"
    )
    # No invalidation, no PreviousInvalidatingCommand fired
    assert len(previous_invalidating_command) == 0

    # Rerun with changes to commandline config
    await buck.build(
        "@root//mode/my_mode",
        "//:test",
        "-c",
        "my_section.my_key=my_new_value",
    )

    previous_invalidating_command = await filter_events(
        buck, "Event", "data", "Instant", "data", "PreviousCommandWithMismatchedConfig"
    )
    assert len(previous_invalidating_command) == 1
    assert previous_invalidating_command[0]["trace_id"] == trace_id
    sanitized_argv = previous_invalidating_command[0]["sanitized_argv"]
    assert (
        # sanitized_argv[0] contains the path to the buck executable which is different on every machine
        sanitized_argv[1] == "build"
        and sanitized_argv[2] == "@root//mode/my_mode"
        and sanitized_argv[3] == "//:test"
        and sanitized_argv[4] == "-c"
        and sanitized_argv[5] == "my_section.my_key=my_value"
    )
    # Make a change to .buckconfig
    with open(buck.cwd / ".buckconfig", "a") as buckconfig:
        buckconfig.write("\n[test_section]\ntest_key = test_value\n")
        await buck.build(
            "@root//mode/my_mode",
            "//:test",
            "-c",
            "my_section.my_key=my_new_value",
        )

    # Previous command didn't change any external configs but still has new_configs_used = 1 due to changes to project-relative configs.
    # We don't capture that by design at the moment.
    record_file = tmp_path / "record.json"
    await buck.build(
        "@root//mode/my_mode",
        "//:test",
        "-c",
        "my_section.my_key=my_new_value",
        "--unstable-write-invocation-record",
        str(record_file),
    )

    previous_invalidating_command = await filter_events(
        buck, "Event", "data", "Instant", "data", "PreviousCommandWithMismatchedConfig"
    )
    assert len(previous_invalidating_command) == 0
    assert read_invocation_record(record_file)["new_configs_used"] == 1
