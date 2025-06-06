/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use async_trait::async_trait;
use buck2_interpreter::factory::ReentrantStarlarkEvaluator;
use dice::DiceComputations;

#[async_trait(?Send)]
pub trait AnonPromisesDyn<'v>: 'v {
    async fn run_promises(
        self: Box<Self>,
        dice: &mut DiceComputations,
        eval: &mut ReentrantStarlarkEvaluator<'_, 'v, '_, '_>,
    ) -> buck2_error::Result<()>;
}
