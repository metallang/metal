// SPDX-License-Identifier: MIT


use metal_ast::SyntaxNode;
use metal_mir::parcel::Module;

pub fn parse_into_module(
    name: String,
    filename: String,
    library: bool,
    syn: SyntaxNode,
) -> Option<Module> {
    todo!()
}
