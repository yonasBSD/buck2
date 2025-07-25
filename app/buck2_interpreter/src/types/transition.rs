/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

use std::sync::Arc;

use buck2_core::configuration::transition::id::TransitionId;
use starlark::any::ProvidesStaticType;
use starlark::values::Value;

#[derive(Debug, buck2_error::Error)]
#[buck2(tag = Input)]
enum TransitionError {
    #[error("cfg parameter is not a transition object: {}", _0)]
    WrongType(String),
}

/// Implemented by starlark transition objects.
pub trait TransitionValue {
    fn transition_id(&self) -> buck2_error::Result<Arc<TransitionId>>;
}

unsafe impl<'v> ProvidesStaticType<'v> for &'v dyn TransitionValue {
    type StaticType = &'static dyn TransitionValue;
}

pub fn transition_id_from_value(value: Value) -> buck2_error::Result<Arc<TransitionId>> {
    match value.request_value::<&dyn TransitionValue>() {
        Some(has) => has.transition_id(),
        None => Err(TransitionError::WrongType(value.to_repr()).into()),
    }
}
