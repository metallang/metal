mod constants;
mod functions;
mod utils;

use constants::codegen_constants;
use functions::codegen_functions;
use metal_ast::Item;

pub fn codegen_wasm(ast: metal_ast::Block<'_>) -> Vec<u8> {
    let mut module = wasm_encoder::Module::new();

    let mut constants = vec![];
    let mut enums = vec![];
    let mut functions = vec![];
    let mut imports = vec![];
    let mut structs = vec![];

    for item in ast.items {
        match item {
            Item::Const(const_item) => constants.push(*const_item),
            Item::Enum(enum_item) => enums.push(*enum_item),
            Item::Expr(_) => unreachable!("top-level expressions have no meaning"),
            Item::Fn(fn_item) => functions.push(*fn_item),
            Item::Import(import_item) => imports.push(*import_item),
            Item::Struct(struct_item) => structs.push(*struct_item),
        }
    }

    codegen_constants(constants, &mut module);
    codegen_functions(functions, &mut module);

    module.finish()
}
