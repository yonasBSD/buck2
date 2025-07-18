/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

use std::time::Duration;

pub trait ProstDurationExt {
    fn try_into_duration(&self) -> buck2_error::Result<Duration>;
}

impl ProstDurationExt for prost_types::Duration {
    fn try_into_duration(&self) -> buck2_error::Result<Duration> {
        Ok(Duration::from_secs(self.seconds.try_into()?)
            + Duration::from_nanos(self.nanos.try_into()?))
    }
}
