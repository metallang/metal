#![feature(decl_macro, new_range_api)]

mod span;

pub use metal_proc_macros::{spanned, Spanned};

pub use crate::span::{span, Span, Spanned};
