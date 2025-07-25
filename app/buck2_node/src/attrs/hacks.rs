/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

//! Utility functions to introspect coerced values. This will go away once we have more of the value
//! coercion tooling done. For now, it handles things like stringification for the `targets` command,
//! converting to JSON, etc.

use buck2_core::package::PackageLabel;

use crate::attrs::coerced_attr::CoercedAttr;
use crate::attrs::configured_attr::ConfiguredAttr;
use crate::attrs::fmt_context::AttrFmtContext;
use crate::attrs::json::ToJsonWithContext;

pub fn value_to_json(
    value: &CoercedAttr,
    pkg: PackageLabel,
) -> buck2_error::Result<serde_json::Value> {
    value.to_json(&AttrFmtContext {
        package: Some(pkg),
        options: Default::default(),
    })
}

pub fn configured_value_to_json(
    value: &ConfiguredAttr,
    pkg: PackageLabel,
) -> buck2_error::Result<serde_json::Value> {
    value.to_json(&AttrFmtContext {
        package: Some(pkg),
        options: Default::default(),
    })
}

pub fn value_to_string(value: &CoercedAttr, pkg: PackageLabel) -> buck2_error::Result<String> {
    match value_to_json(value, pkg)?.as_str() {
        Some(s) => Ok(s.to_owned()),
        None => Err(buck2_error::buck2_error!(
            buck2_error::ErrorTag::Input,
            "Expected a string, did not get one",
        )),
    }
}
