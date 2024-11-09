use metal_ast::FnItem;
use wasm_encoder::{CodeSection, Function, FunctionSection, Instruction, Module};

use crate::utils::ty_to_wasm_type;

pub fn codegen_functions(functions: Vec<FnItem<'_>>, module: &mut Module) {
    let mut header = FunctionSection::new();
    let mut body = CodeSection::new();

    for (idx, function) in functions.into_iter().enumerate() {
        header.function(idx as u32);

        body.function(&codegen_function(&function));
    }

    module.section(&header);
    module.section(&body);
}

fn codegen_function(function: &FnItem<'_>) -> Function {
    let wasm_inputs = function
        .inputs
        .iter()
        .map(|i| {
            i.ty.as_ref()
                .unwrap_or_else(|| panic!("expected {i:?} to have a type"))
        })
        .map(|ty| {
            ty_to_wasm_type(ty).unwrap_or_else(|| panic!("expected {ty:?} to be a valid type"))
        });

    let mut func = Function::new_with_locals_types(wasm_inputs);

    for _instruction in &function.body.items {
        todo!()
    }

    func.instruction(&Instruction::End);

    func
}
