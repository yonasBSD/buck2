/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

impl From<std::string::FromUtf8Error> for crate::Error {
    #[cold]
    #[track_caller]
    fn from(value: std::string::FromUtf8Error) -> Self {
        crate::conversion::from_any(value)
    }
}

impl From<std::string::String> for crate::Error {
    #[cold]
    #[track_caller]
    fn from(value: std::string::String) -> Self {
        let source_location =
            crate::source_location::from_file(std::panic::Location::caller().file(), None);

        crate::Error::new(value, source_location, None)
    }
}
