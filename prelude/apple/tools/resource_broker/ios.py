# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is dual-licensed under either the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree or the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree. You may select, at your option, one of the
# above-listed licenses.

# pyre-strict

import os
from dataclasses import dataclass
from typing import List, Optional

from packaging.version import Version

from .idb_target import (
    IdbTarget,
    managed_simulators_list_from_stdout,
    SimState,
    SimulatorInfo,
)

from .simctl_runtime import list_runtimes, XCSimRuntime

from .timeouts import SIMULATOR_BOOT_TIMEOUT

from .utils import execute_generic_text_producing_command


@dataclass(frozen=True)
class SimulatorSpec:
    os_version: str
    device: str


def _device_set_path() -> str:
    return os.path.expanduser("~/Library/Developer/Buck2IdbDeviceSet")


def _list_managed_simulators_command(simulator_manager: str) -> List[str]:
    return [
        simulator_manager,
        "list",
        "--device-set-path",
        _device_set_path(),
        "--only",
        "simulator",
    ]


def _create_simulator_command(simulator_manager: str, sim_spec: str) -> List[str]:
    return [
        simulator_manager,
        "create",
        "--device-set-path",
        _device_set_path(),
        "--configuration",
        sim_spec,
    ]


def _boot_simulator_command(simulator_manager: str, udid: str) -> List[str]:
    return [
        simulator_manager,
        "boot",
        "--device-set-path",
        _device_set_path(),
        udid,
    ]


def _compatible_device_type_from_runtime(
    runtime: XCSimRuntime, device: Optional[str]
) -> Optional[str]:
    device_types = list(runtime.supported_device_types)
    if device:
        device_types = [t for t in device_types if t.name == device]
    if not device_types:
        return None
    default = device_types[0]
    choice = next(
        (
            device_type.name
            for device_type in device_types
            if device_type.name == "iPhone 11"
        ),
        default.name,
    )
    return choice


def _select_simulator_spec(
    runtimes: List[XCSimRuntime], os_version: Optional[str], device: Optional[str]
) -> SimulatorSpec:
    runtimes.sort(key=lambda x: Version(x.version), reverse=True)
    if os_version:
        runtimes = [x for x in runtimes if x.name == os_version]
    for runtime in runtimes:
        device_type = _compatible_device_type_from_runtime(runtime, device)
        if device_type:
            if device_type.startswith("Apple Watch"):
                return SimulatorSpec(f"watchOS {runtime.version}", device_type)
            else:
                return SimulatorSpec(f"iOS {runtime.version}", device_type)
    raise RuntimeError(
        "No Xcode simctl compatible os runtime and device available. Try to `sudo xcode-select -s <path_to_xcode>` and *open Xcode to install all required components*."
    )


async def _generic_managed_simulators_list_command(
    name: str, cmd: List[str]
) -> List[IdbTarget]:
    stdout = await execute_generic_text_producing_command(name=name, cmd=cmd)
    return managed_simulators_list_from_stdout(stdout)


async def _list_managed_simulators(simulator_manager: str) -> List[IdbTarget]:
    list_cmd = _list_managed_simulators_command(simulator_manager=simulator_manager)
    return await _generic_managed_simulators_list_command(
        name="list managed simulators", cmd=list_cmd
    )


def normalize_os_version(os_version: str) -> Version:
    # os version should be in the format "iOS 17.2.0" or "iOS 17.2"
    return Version(os_version.split(" ")[1])


def choose_simulators(
    simulators: List[IdbTarget], os_version: Optional[str], device: Optional[str]
) -> List[IdbTarget]:
    # If no device or os_version is specified, default to only iPhone simulators (where os_version starts with "iOS")
    if not device and not os_version:
        simulators = list(
            filter(
                lambda s: (s.os_version.startswith("iOS")),
                simulators,
            )
        )

    filtered_simulators = filter(
        lambda s: (
            (
                normalize_os_version(s.os_version).major
                == normalize_os_version(os_version).major
                and normalize_os_version(s.os_version).minor
                == normalize_os_version(os_version).minor
                if os_version
                else True
            )
            and (s.name == device if device is not None else True)
        ),
        simulators,
    )

    return list(filtered_simulators)


async def _create_simulator(
    simulator_manager: str,
    os_version: Optional[str] = None,
    device: Optional[str] = None,
) -> None:
    runtimes = await list_runtimes()
    spec = _select_simulator_spec(runtimes, os_version, device)
    spec_str = f"{spec.device},{spec.os_version}"
    create_cmd = _create_simulator_command(
        simulator_manager=simulator_manager, sim_spec=spec_str
    )
    await execute_generic_text_producing_command(
        name="create simulators", cmd=create_cmd
    )


async def _get_managed_simulators_create_if_needed(
    simulator_manager: str,
    os_version: Optional[str] = None,
    device: Optional[str] = None,
) -> List[IdbTarget]:
    managed_simulators = await _get_managed_simulators(
        simulator_manager=simulator_manager, os_version=os_version, device=device
    )
    if managed_simulators:
        return managed_simulators

    await _create_simulator(
        simulator_manager=simulator_manager, os_version=os_version, device=device
    )
    managed_simulators = await _get_managed_simulators(
        simulator_manager=simulator_manager, os_version=os_version, device=device
    )
    if managed_simulators:
        return managed_simulators

    raise RuntimeError(
        "Failed to create a simulator. Try to `sudo xcode-select -s <path_to_xcode>` and *open Xcode to install all required components*."
    )


async def _get_managed_simulators(
    simulator_manager: str,
    os_version: Optional[str] = None,
    device: Optional[str] = None,
) -> List[IdbTarget]:
    managed_simulators = await _list_managed_simulators(
        simulator_manager=simulator_manager
    )
    return choose_simulators(managed_simulators, os_version, device)


def _select_simulator(
    only_booted: bool, all_simulators: List[IdbTarget]
) -> Optional[IdbTarget]:
    return next(
        filter(
            lambda s: s.state == SimState.booted if only_booted else True,
            iter(all_simulators),
        ),
        None,
    )


def _select_simulator_with_preference(
    prefer_booted: bool, all_simulators: List[IdbTarget]
) -> IdbTarget:
    simulator = _select_simulator(
        only_booted=prefer_booted, all_simulators=all_simulators
    )
    if not simulator and prefer_booted:
        simulator = _select_simulator(only_booted=False, all_simulators=all_simulators)
    if not simulator:
        raise RuntimeError("Expected at least unbooted simulator entity to be selected")
    return simulator


async def prepare_simulator(
    simulator_manager: str,
    booted: bool,
    os_version: Optional[str] = None,
    device: Optional[str] = None,
) -> SimulatorInfo:
    managed_simulators = await _get_managed_simulators_create_if_needed(
        simulator_manager=simulator_manager,
        os_version=os_version,
        device=device,
    )

    simulator = _select_simulator_with_preference(
        prefer_booted=booted, all_simulators=managed_simulators
    )

    if simulator.state != SimState.booted and booted:
        boot_cmd = _boot_simulator_command(
            simulator_manager=simulator_manager, udid=simulator.udid
        )
        await execute_generic_text_producing_command(
            name="boot simulator",
            cmd=boot_cmd,
            timeout=SIMULATOR_BOOT_TIMEOUT,
        )

    return SimulatorInfo(
        udid=simulator.udid,
        device_set_path=_device_set_path(),
    )
