/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

#![cfg(test)]
#![allow(dead_code)]

mod bounds;
mod dst;
mod skip;
mod visit;
mod with_flamegraph;

use crate as allocative;
use crate::Allocative;

#[derive(Allocative)]
struct Empty {}

#[derive(Allocative)]
struct TupleStruct(u32, String);

#[derive(Allocative)]
struct RegularStruct {
    a: u32,
    b: String,
}

#[derive(Allocative)]
enum Enum {
    Unit,
    Tuple(u32, String),
    Regular { a: u32, b: String },
}

#[derive(Allocative)]
enum GenericEnum<T> {
    Unit,
    Tuple(T, String),
}

#[derive(Allocative)]
struct StructWithDefaultParam<T = String>(T);
