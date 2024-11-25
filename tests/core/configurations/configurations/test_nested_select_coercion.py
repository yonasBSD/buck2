# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under both the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree and the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree.

# pyre-strict


from buck2.tests.e2e_util.api.buck import Buck
from buck2.tests.e2e_util.asserts import expect_failure
from buck2.tests.e2e_util.buck_workspace import buck_test


@buck_test()
async def test_nested_select_coercion(buck: Buck) -> None:
    # TODO(T208519965): Fix the issue
    await expect_failure(
        buck.uquery("//:foo"),
        stderr_regex="Expected value of type `bool`, got value with type `NoneType`",
    )