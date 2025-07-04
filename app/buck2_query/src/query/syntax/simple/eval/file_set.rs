/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

use std::fmt;

use allocative::Allocative;
use buck2_core::cells::cell_path::CellPath;
use derive_more::Display;
use display_container::fmt_container;
use fancy_regex::Regex;
use indexmap::IndexSet;

use crate::query::environment::QueryEnvironment;
use crate::query::environment::QueryTarget;
use crate::query::syntax::simple::eval::error::QueryError;
use crate::query::syntax::simple::eval::set::TargetSet;

/// An entry in a FileSet.
#[derive(Hash, Eq, PartialEq, Debug, Clone, Display, Allocative)]
pub struct FileNode(pub CellPath);

#[derive(Debug, Eq, PartialEq, Clone, Allocative)]
pub struct FileSet {
    files: IndexSet<FileNode>,
}

impl fmt::Display for FileSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_container(f, "[", "]", self.files.iter())
    }
}

impl FileSet {
    pub fn new(files: IndexSet<FileNode>) -> Self {
        Self { files }
    }

    pub(crate) fn filter_name(&self, regex: &str) -> buck2_error::Result<Self> {
        let re = Regex::new(regex)?;
        self.filter(|node| Ok(re.is_match(&node.0.to_string())?))
    }

    fn filter<F: Fn(&FileNode) -> buck2_error::Result<bool>>(
        &self,
        filter: F,
    ) -> buck2_error::Result<Self> {
        let mut files = IndexSet::new();
        for file in self.files.iter() {
            if filter(file)? {
                files.insert(file.clone());
            }
        }
        Ok(Self { files })
    }

    pub fn insert(&mut self, file: FileNode) {
        self.files.insert(file);
    }

    pub fn insert_all(&mut self, other: &FileSet) {
        for v in other.files.iter() {
            self.files.insert(v.clone());
        }
    }

    pub fn union(&self, right: &FileSet) -> FileSet {
        let mut files = self.files.clone();
        for file in right.files.iter() {
            files.insert(file.clone());
        }
        Self { files }
    }

    pub fn owner<T: QueryTarget>(
        &self,
        _env: &impl QueryEnvironment<Target = T>,
    ) -> buck2_error::Result<TargetSet<T>> {
        Err(QueryError::FunctionUnimplemented("owner()").into())
    }

    pub fn iter(&self) -> impl Iterator<Item = &CellPath> + '_ {
        self.files.iter().map(|v| &v.0)
    }

    pub fn get_index(&self, index: usize) -> Option<&CellPath> {
        match self.files.get_index(index) {
            Some(x) => Some(&x.0),
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }

    pub fn contains(&self, item: &FileNode) -> bool {
        self.files.contains(item)
    }

    pub fn intersect(&self, right: &FileSet) -> buck2_error::Result<FileSet> {
        self.filter(|file| Ok(right.contains(file)))
    }

    pub fn difference(&self, right: &FileSet) -> buck2_error::Result<FileSet> {
        self.filter(|file| Ok(!right.contains(file)))
    }
}

impl FromIterator<FileNode> for FileSet {
    fn from_iter<T: IntoIterator<Item = FileNode>>(iter: T) -> FileSet {
        FileSet {
            files: IndexSet::from_iter(iter),
        }
    }
}
