/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

use buck2_node::attrs::attr_type::configured_dep::ExplicitConfiguredDepAttrType;
use buck2_node::attrs::attr_type::configured_dep::UnconfiguredExplicitConfiguredDep;
use buck2_node::attrs::attr_type::dep::DepAttrType;
use buck2_node::attrs::coerced_attr::CoercedAttr;
use buck2_node::attrs::coercion_context::AttrCoercionContext;
use buck2_node::attrs::configurable::AttrIsConfigurable;
use dupe::Dupe;
use starlark::typing::Ty;
use starlark::values::UnpackValue;
use starlark::values::Value;

use crate::attrs::coerce::AttrTypeCoerce;
use crate::attrs::coerce::attr_type::ty_maybe_select::TyMaybeSelect;

impl AttrTypeCoerce for DepAttrType {
    fn coerce_item(
        &self,
        _configurable: AttrIsConfigurable,
        ctx: &dyn AttrCoercionContext,
        value: Value,
    ) -> buck2_error::Result<CoercedAttr> {
        let label = ctx.coerce_providers_label(value.unpack_str_err()?)?;

        Ok(CoercedAttr::Dep(label))
    }

    fn starlark_type(&self) -> TyMaybeSelect {
        TyMaybeSelect::Basic(Ty::string())
    }
}

impl AttrTypeCoerce for ExplicitConfiguredDepAttrType {
    fn coerce_item(
        &self,
        _configurable: AttrIsConfigurable,
        ctx: &dyn AttrCoercionContext,
        value: Value,
    ) -> buck2_error::Result<CoercedAttr> {
        let (label_string, platform_string): (&str, &str) = UnpackValue::unpack_value_err(value)?;

        let label = ctx.coerce_providers_label(label_string)?;

        let platform = ctx.coerce_target_label(platform_string)?;

        Ok(CoercedAttr::ExplicitConfiguredDep(Box::new(
            UnconfiguredExplicitConfiguredDep {
                attr_type: self.dupe(),
                label,
                platform,
            },
        )))
    }

    fn starlark_type(&self) -> TyMaybeSelect {
        TyMaybeSelect::Tuple(vec![
            TyMaybeSelect::Basic(Ty::string()),
            TyMaybeSelect::Basic(Ty::string()),
        ])
    }
}
