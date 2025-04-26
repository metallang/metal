// SPDX-License-Identifier: MIT

use std::sync::{Arc, RwLock};

use metal_ast::AstNode;
use metal_mir::{
    expr::{literals::Bool, Expr},
    stmt::constant::Constant,
    types::{primitives::Primitive, visibility::Visibility},
};

use crate::functions::Function;

pub struct Statement;

impl Statement {
    pub fn parse(
        module: &Arc<RwLock<metal_mir::parcel::Module>>,
        stmt: metal_ast::StmtNode,
    ) -> Option<()> {
        if let metal_ast::StmtKindNode::Item(item) = stmt.kind_node()? {
            match item.kind_node()? {
                metal_ast::ItemKindNode::ConstItem(node) => {
                    let mir = metal_mir::stmt::Statement::Constant(Box::new(Constant {
                        name: node.name_node()?.syntax().to_string(),
                        expr: Expr::Literal(Box::new(metal_mir::expr::literals::Literal::Boolean(
                            Bool { value: true },
                        ))),
                        ty: Primitive::I8,
                        vis: metal_mir::types::visibility::Visibility::Public,
                    }));
                    {
                        let mut md = module.write().unwrap();
                        md.statements.push(Box::new(mir));
                    };
                    Some(())
                }

                metal_ast::ItemKindNode::FnItem(node) => {
                    let vin = item.vis_node();
                    let vis = if vin.is_some() {
                        Visibility::Public
                    } else {
                        Visibility::Private
                    };
                    Function::new(module).parse(node, vis);
                    Some(())
                }

                // structs and enums are only *types* in MIR and never represented as statements.
                metal_ast::ItemKindNode::StructItem(_) => unreachable!(),
                metal_ast::ItemKindNode::EnumItem(_) => unreachable!(),

                metal_ast::ItemKindNode::ImportItem(_) => todo!(),
                metal_ast::ItemKindNode::TypeAliasItem(_) => todo!(),
                _ => None,
            }
        } else {
            None
        }
    }
}
