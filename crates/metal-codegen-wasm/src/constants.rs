use metal_ast::ConstItem;
use wasm_encoder::{GlobalSection, GlobalType, Module, ValType};

use crate::utils::{expr_to_wasm_constexpr, expr_to_wasm_type, ty_to_wasm_type};

pub fn codegen_constants(constants: Vec<ConstItem<'_>>, module: &mut Module) {
    let mut section = GlobalSection::new();

    for constant in constants {
        let val_type = constant_valtype(&constant);

        let global_type = GlobalType {
            val_type,
            mutable: false,
            shared: false,
        };

        let init_expr = expr_to_wasm_constexpr(&constant.value)
            .unwrap_or_else(|| panic!("expected {:?} to be a valid const val", &constant.value));

        section.global(global_type, &init_expr);
    }

    module.section(&section);
}

fn constant_valtype(constant: &ConstItem<'_>) -> ValType {
    if let Some(ty) = constant.ty.as_ref() {
        ty_to_wasm_type(ty).unwrap_or_else(|| panic!("expected {ty:?} to be a valid type"))
    } else {
        let expr = &constant.value;

        expr_to_wasm_type(expr)
            .unwrap_or_else(|| panic!("expected {expr:?} to be a valid const val"))
    }
}
