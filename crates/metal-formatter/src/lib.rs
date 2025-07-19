// SPDX-License-Identifier: MIT
#![feature(default_field_values, ptr_as_ref_unchecked, likely_unlikely)]
mod build;
mod dom;
mod error;
mod helpers;
// mod layout;
mod render;

pub use crate::{
    dom::{build::DomNodeBuilder, BreakIf, Dom, DomNode, DomNodeKind, RenderIf},
    error::{Error, Result},
    // layout::LayoutState,
};
