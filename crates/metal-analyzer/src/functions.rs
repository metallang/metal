// SPDX-License-Identifier: MIT

use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use metal_ast::AstNode;

use crate::types::Type;

pub struct Function<'a> {
    pub module: &'a Arc<RwLock<metal_mir::parcel::Module>>,
    pub def: Option<metal_mir::stmt::functiondef::FunctionDefinition>,
}

impl<'a> Function<'a> {
    pub fn new(module: &'a Arc<RwLock<metal_mir::parcel::Module>>) -> Self {
        Self { module, def: None }
    }

    pub fn parse(
        &mut self,
        fun: metal_ast::FnItemNode,
        vis: metal_mir::types::visibility::Visibility,
    ) -> Option<&Self> {
        let name = fun.name_node()?.syntax().to_string();
        let sig_syn = fun.sig_node()?;

        let mut return_type = Type::new(self.module);
        return_type.parse(&sig_syn.return_ty_node()?.type_node()?);
        let mut inputs = BTreeMap::new();
        sig_syn.inputs_node()?.children().for_each(|ty| {
            inputs.insert(
                ty.name_node().unwrap().syntax().to_string(),
                Type::new(self.module)
                    .parse(&ty.ty_node().unwrap().type_node().unwrap())
                    .internal
                    .clone()
                    .expect("Input type could not be found"),
            );
        });

        let signature = metal_mir::types::function::FunctionSignature {
            name,
            return_type: return_type
                .internal
                .expect("Return type could not be found"),
            inputs,
            vis,
        };
        self.def = Some(metal_mir::stmt::functiondef::FunctionDefinition {
            signature,
            body: Vec::new(),
        });

        {
            let mut md = self.module.write().unwrap();
            md.statements
                .push(Box::new(metal_mir::stmt::Statement::FunctionDefine(
                    Box::new(self.def.clone().unwrap()),
                )));
        }

        Some(self)
    }
}
