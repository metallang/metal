// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

use metal_ast::{Expr, LitExpr, Ty};
use wasm_encoder::{ConstExpr, ValType};

pub fn ty_to_wasm_type(ty: &Ty<'_>) -> Option<ValType> {
    match ty {
        Ty::Ident(ident) => str_to_wasm_type(ident.inner),
    }
}

pub fn expr_to_wasm_type(expr: &Expr<'_>) -> Option<ValType> {
    Some(match expr {
        Expr::Ident(_) => return None, // TODO
        Expr::Lit(lit_expr) => match lit_expr.as_ref() {
            LitExpr::Bool(_) => ValType::I32,
            LitExpr::Num(_) => ValType::F64,
            LitExpr::Str(_) => return None, // TODO
        },
        Expr::Call(_) => return None, // TODO
    })
}

pub fn expr_to_wasm_constexpr(expr: &Expr<'_>) -> Option<ConstExpr> {
    Some(match expr {
        Expr::Ident(_) => todo!(),
        Expr::Lit(lit_expr) => match lit_expr.as_ref() {
            LitExpr::Bool(bool_lit) => ConstExpr::i32_const(bool_lit.inner as i32),
            LitExpr::Num(num_lit) => ConstExpr::f64_const(num_lit.inner),
            LitExpr::Str(_) => return None, // TODO
        },
        _ => return None, // TODO
    })
}

fn str_to_wasm_type(s: &str) -> Option<ValType> {
    Some(match s {
        "i32" | "bool" => ValType::I32,
        "i64" => ValType::I64,
        "f32" => ValType::I32,
        "f64" => ValType::F64,
        _ => return None,
    })
}
