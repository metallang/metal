// SPDX-License-Identifier: MIT
#![feature(
    default_field_values,
    ptr_as_ref_unchecked,
    likely_unlikely,
    decl_macro
)]
mod build;
mod dom;
mod error;
mod helpers;
mod utils;

const INDENT: &str = "    ";
const COLUMN_LIMIT: usize = 10;
const NEWLINE: &str = "\n";

pub use crate::{
    dom::{
        build::DomNodeBuilder,
        children::{mut_::DomNodeMut, ref_::DomNode},
        BreakIf, Dom, DomNodeData, DomNodeKind, RenderIf,
    },
    error::{Error, Result},
};
