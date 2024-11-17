// SPDX-License-Identifier: MIT

#![feature(decl_macro, stmt_expr_attributes, new_range_api)]

mod block;
mod delimiter;
mod expr;
mod item;
mod misc;
mod ty;
mod utils;

pub use crate::{
    block::{parse_block, parse_block_raw},
    delimiter::Delimiter,
    expr::{lit::parse_lit_expr, parse_expr, parse_expr_specifier},
    item::{
        const_item::parse_const_item,
        enum_item::{parse_enum_body, parse_enum_item, parse_enum_variant},
        fn_item::{parse_fn_input, parse_fn_item},
        import::{
            parse_import_item, parse_import_tree, parse_multi_import,
            parse_name_or_segment_import_tree,
        },
        parse_item,
        struct_item::{parse_struct_body, parse_struct_field, parse_struct_item},
    },
    misc::{parse_ident, parse_mutness, parse_vis},
    ty::{parse_ty, parse_ty_qualifier},
    utils::{expect, parse_delimited, parser_type},
};
