# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is dual-licensed under either the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree or the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree. You may select, at your option, one of the
# above-listed licenses.

# pyre-strict


from buck2.tests.e2e_util.api.buck import Buck
from buck2.tests.e2e_util.buck_workspace import buck_test


@buck_test()
async def test_read_root_config(buck: Buck) -> None:
    output = await buck.build("//:")
    assert "<<root=regular>>" in output.stderr
    assert "<<root_ignore_default=regular>>" in output.stderr
    assert "<<root_use_default=predict>>" in output.stderr
    assert "<<local=regular>>" in output.stderr

    output = await buck.build("other//:")
    assert "{{root=regular}}" in output.stderr
    assert "{{root_ignore_default=regular}}" in output.stderr
    assert "{{root_use_default=quantity}}" in output.stderr
    assert "{{local=guerrilla}}" in output.stderr
