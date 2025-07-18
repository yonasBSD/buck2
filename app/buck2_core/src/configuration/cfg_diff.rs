/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::Write;

use crate::configuration::constraints::ConstraintKey;
use crate::configuration::constraints::ConstraintValue;
use crate::configuration::data::ConfigurationData;
use crate::configuration::data::ConfigurationDataData;

/// If configurations are not equal, return difference.
pub fn cfg_diff(a: &ConfigurationData, b: &ConfigurationData) -> Result<(), String> {
    if a == b {
        return Ok(());
    }

    #[derive(Default)]
    struct DiffPrinter {
        s: String,
    }

    impl DiffPrinter {
        fn print_diff_line(&mut self, sign: char, line: &str) {
            writeln!(self.s, "{sign} {line}").unwrap();
        }

        fn print_label_line(&mut self, sign: char, label: &str) {
            self.print_diff_line(sign, &format!("label: {label}"));
        }

        fn print_label_result_line(&mut self, sign: char, label: buck2_error::Result<&str>) {
            match label {
                Ok(label) => self.print_label_line(sign, label),
                Err(e) => self.print_diff_line(sign, &format!("label error: {e}")),
            }
        }

        fn print_cfg_data_result_line(
            &mut self,
            sign: char,
            data: buck2_error::Result<&ConfigurationDataData>,
        ) {
            match data {
                Ok(_) => self.print_diff_line(sign, "data"),
                Err(e) => self.print_diff_line(sign, &format!("data error: {e}")),
            }
        }

        fn diff_label_result(
            &mut self,
            a: buck2_error::Result<&str>,
            b: buck2_error::Result<&str>,
        ) {
            match (a, b) {
                (Ok(a), Ok(b)) => {
                    if a != b {
                        self.print_label_line('-', a);
                        self.print_label_line('+', b);
                    }
                }
                (a, b) => {
                    // There should be no errors because configurations are bound, but better than panic.
                    self.print_label_result_line('-', a);
                    self.print_label_result_line('+', b);
                }
            }
        }

        fn diff_btree_map<K: Ord, V: PartialEq>(
            &mut self,
            a: &BTreeMap<K, V>,
            b: &BTreeMap<K, V>,
            fmt: impl Fn(&K, &V) -> String,
        ) {
            let mut a = a.iter().peekable();
            let mut b = b.iter().peekable();
            loop {
                match (a.peek(), b.peek()) {
                    (Some(an), Some(bn)) => match an.0.cmp(bn.0) {
                        Ordering::Equal => {
                            if an.1 != bn.1 {
                                self.print_diff_line('-', &fmt(an.0, an.1));
                                self.print_diff_line('+', &fmt(bn.0, bn.1));
                            }
                            a.next().unwrap();
                            b.next().unwrap();
                        }
                        Ordering::Less => {
                            self.print_diff_line('-', &fmt(an.0, an.1));
                            a.next().unwrap();
                        }
                        Ordering::Greater => {
                            self.print_diff_line('+', &fmt(bn.0, bn.1));
                            b.next().unwrap();
                        }
                    },
                    (Some(an), None) => {
                        self.print_diff_line('-', &fmt(an.0, an.1));
                        a.next().unwrap();
                    }
                    (None, Some(bn)) => {
                        self.print_diff_line('+', &fmt(bn.0, bn.1));
                        b.next().unwrap();
                    }
                    (None, None) => break,
                }
            }
        }

        fn diff_constraints(
            &mut self,
            a: &BTreeMap<ConstraintKey, ConstraintValue>,
            b: &BTreeMap<ConstraintKey, ConstraintValue>,
        ) {
            self.diff_btree_map(a, b, |k, v| format!("constraint: {k} -> {v}"))
        }

        fn diff_cfg_data(&mut self, a: &ConfigurationDataData, b: &ConfigurationDataData) {
            self.diff_constraints(&a.constraints, &b.constraints);
        }

        fn diff_cfg_data_result(
            &mut self,
            a: buck2_error::Result<&ConfigurationDataData>,
            b: buck2_error::Result<&ConfigurationDataData>,
        ) {
            match (a, b) {
                (Ok(a), Ok(b)) => self.diff_cfg_data(a, b),
                (a, b) => {
                    self.print_cfg_data_result_line('-', a);
                    self.print_cfg_data_result_line('+', b);
                }
            }
        }
    }

    let mut diff = DiffPrinter::default();
    diff.diff_label_result(a.label(), b.label());
    diff.diff_cfg_data_result(a.data(), b.data());

    Err(diff.s)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::configuration::cfg_diff::cfg_diff;
    use crate::configuration::constraints::ConstraintKey;
    use crate::configuration::constraints::ConstraintValue;
    use crate::configuration::data::ConfigurationData;
    use crate::configuration::data::ConfigurationDataData;
    use crate::target::label::label::TargetLabel;

    #[test]
    fn test_diff() {
        let x = ConfigurationData::from_platform(
            "xx".to_owned(),
            ConfigurationDataData::new(BTreeMap::from_iter([
                (
                    ConstraintKey(TargetLabel::testing_parse("foo//bar:c")),
                    ConstraintValue(TargetLabel::testing_parse("foo//bar:v")),
                ),
                (
                    ConstraintKey(TargetLabel::testing_parse("foo//qux:c")),
                    ConstraintValue(TargetLabel::testing_parse("foo//qux:vx")),
                ),
            ])),
        )
        .unwrap();
        let y = ConfigurationData::from_platform(
            "yy".to_owned(),
            ConfigurationDataData::new(BTreeMap::from_iter([
                (
                    ConstraintKey(TargetLabel::testing_parse("foo//bar:c")),
                    ConstraintValue(TargetLabel::testing_parse("foo//bar:v")),
                ),
                (
                    ConstraintKey(TargetLabel::testing_parse("foo//baz:c")),
                    ConstraintValue(TargetLabel::testing_parse("foo//baz:vy")),
                ),
                (
                    ConstraintKey(TargetLabel::testing_parse("foo//qux:c")),
                    ConstraintValue(TargetLabel::testing_parse("foo//qux:vy")),
                ),
            ])),
        )
        .unwrap();
        let diff = cfg_diff(&x, &y).unwrap_err();
        assert_eq!(
            "\
            - label: xx\n\
            + label: yy\n\
            + constraint: foo//baz:c -> foo//baz:vy\n\
            - constraint: foo//qux:c -> foo//qux:vx\n\
            + constraint: foo//qux:c -> foo//qux:vy\n\
            ",
            diff
        );
    }
}
