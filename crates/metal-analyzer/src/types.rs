// SPDX-License-Identifier: MIT

use std::sync::{Arc, RwLock};

use map_macro::hash_map;
use metal_ast::AstNode;
use metal_mir::types::primitives::Primitive;

pub struct Type<'a> {
    pub module: &'a Arc<RwLock<metal_mir::parcel::Module>>,
    pub internal: Option<metal_mir::types::Type>,
}

impl<'a> Type<'a> {
    pub fn new(module: &'a Arc<RwLock<metal_mir::parcel::Module>>) -> Self {
        Self {
            module,
            internal: None,
        }
    }

    pub fn parse(&mut self, ty: &metal_ast::TypeNode) -> &Self {
        self.simple_check(ty)
    }

    pub fn simple_check(&mut self, ty: &metal_ast::TypeNode) -> &Self {
        if let metal_ast::TypeNode::NameType(node) = ty {
            // TODO: generics
            if node.generics_node().is_some() {
                return self;
            };

            let real_node = node.clone().syntax().text().to_string();

            let primitives = hash_map! {
                "i8" => Primitive::I8,
                "i16" => Primitive::I16,
                "i32" => Primitive::I32,
                "i64" => Primitive::I64,
                "i128" => Primitive::I128,

                "u8" => Primitive::U8,
                "u16" => Primitive::U16,
                "u32" => Primitive::U32,
                "u64" => Primitive::U64,
                "u128" => Primitive::U128,

                "f32" => Primitive::F32,
                "f64" => Primitive::F64,
                "f128" => Primitive::F128,

                "void" => Primitive::Void
            };

            // primitive types
            if primitives.keys().any(|i| i == &real_node.as_str()) {
                let prim = *primitives.get(&real_node.as_str()).unwrap();

                self.internal = Some(metal_mir::types::Type::Primitive(Box::new(prim)));
                return self;
            };
            self
        } else {
            self
        }
    }
}
