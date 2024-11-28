// SPDX-License-Identifier: MIT
//! Compilation step for modules from MIR to LLVM IR

use std::{borrow::Cow, ffi::CString};

use llvm_sys::{analysis::LLVMVerifyModule, bit_writer, core::LLVMPrintModuleToString};
use metal_mir::parcel::Module;

use crate::{
    safeties::{LLVMErrorMessage, MemoryBuffer},
    CodeGenValue, LLVMRefs,
};

// TODO: move to AST once ast-ng is complete.
pub struct PathBuilder<'a> {
    inner: Vec<Cow<'a, str>>,
}

impl Default for PathBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> PathBuilder<'a> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, content: &'a str) {
        self.inner.push(Cow::from(content));
    }

    pub fn finish(&mut self) -> String {
        self.inner.reverse();
        self.inner.join(".")
    }
}

pub fn get_module_full_name(module: &Module) -> String {
    let mut builder = PathBuilder::new();
    let mut last_module = &Some(module);

    while let Some(module) = last_module {
        builder.push(module.name.as_str());
        last_module = &module.parent;
    }

    builder.finish()
}

/// Compiles an LLVM module and returns either human-readable
/// LLVM IR or LLVM bytecode depending on `human_readable`.
pub fn compile_module(module: &Module, human_readable: bool) -> Vec<u8> {
    let mut llvm = LLVMRefs::new(module);

    for stmt in &module.statements {
        stmt.llvm_value(&mut llvm, module);
    }

    let error = LLVMErrorMessage::new();
    let valid = unsafe {
        LLVMVerifyModule(
            llvm.module,
            llvm_sys::analysis::LLVMVerifierFailureAction::LLVMPrintMessageAction,
            &mut error.llvm(),
        )
    };

    if valid != 0 {
        panic!("LLVM compilation error: \n\n{}", error.message());
    }

    if human_readable {
        let module_ir = unsafe { CString::from_raw(LLVMPrintModuleToString(llvm.module)) };
        module_ir.into_bytes()
    } else {
        let buf = unsafe { bit_writer::LLVMWriteBitcodeToMemoryBuffer(llvm.module) };
        MemoryBuffer::new(buf).to_vec()
    }
}

#[cfg(test)]
mod tests {
    use metal_mir::{
        expr::{
            literals::{Literal, Number},
            Assignment, Expr, MathematicalValue,
        },
        stmt::{constant::Constant, functiondef::FunctionDefinition, Statement},
        types::{function::FunctionSignature, primitives::Primitive, Type},
    };

    use super::*;

    fn get_empty_module<'a>(name: String, parent: Option<&'a Module<'a>>) -> Module<'a> {
        Module {
            name,
            parent: parent.map(Box::new),
            children: Vec::new(),
            statements: Vec::new(),
            function_signatures: Vec::new(),
            imports: Vec::new(),
            constants: Vec::new(),
            structs: Vec::new(),
        }
    }

    #[test]
    fn test_empty_module() {
        let module = get_empty_module("empty".to_string(), None);
        let compiled = String::from_utf8(compile_module(&module, true)).unwrap();
        assert_ne!(compiled, "".to_string());
    }

    #[test]
    fn test_module_function() {
        let mut module = get_empty_module("testing".to_string(), None);

        // add function to module
        let sig = FunctionSignature {
            name: "main".to_string(),
            return_type: Type::Primitive(Box::new(Primitive::Void)),
            inputs: Vec::new(),
            vis: metal_mir::types::visibility::Visibility::Public,
        };
        let def = FunctionDefinition {
            module: module.clone(),
            signature: sig.clone(),
            body: [
                Statement::Let(Box::new(Assignment {
                    name: "a_variable",
                    ty: Type::Primitive(Box::new(Primitive::U8)),
                    expr: Some(Expr::Add(Box::new(MathematicalValue {
                        a: Expr::Literal(Box::new(Literal::Number(Number {
                            primitive: Primitive::U8,
                            value: 5,
                        }))),
                        b: Expr::Literal(Box::new(Literal::Number(Number {
                            primitive: Primitive::U8,
                            value: 8,
                        }))),
                        signed: false,
                        float: false,
                        result_var_name: Some("a_variable"),
                    }))),
                })),
                Statement::Return(None),
            ]
            .to_vec(),
        };
        let stmt = Statement::FunctionDefine(Box::new(def));

        module.function_signatures.push(&sig);
        module.statements.push(&stmt);

        let compiled = String::from_utf8(compile_module(&module, true)).unwrap();
        assert_ne!(compiled, "".to_string());
    }

    #[test]
    #[should_panic]
    fn test_panic_global_var_expr() {
        let mut module = get_empty_module("testing".to_string(), None);
        let global_var = Constant {
            name: "some_constant",
            expr: Expr::Div(Box::new(MathematicalValue {
                a: Expr::Literal(Box::new(Literal::Number(Number {
                    primitive: Primitive::I32,
                    value: 40,
                }))),
                b: Expr::Literal(Box::new(Literal::Number(Number {
                    primitive: Primitive::I32,
                    value: 40,
                }))),
                signed: true,
                float: false,
                result_var_name: Some("some_constant"),
            })),
            ty: Primitive::I32,
            vis: metal_mir::types::visibility::Visibility::Private,
        };
        let stmt = Statement::Constant(Box::new(global_var));

        module.statements.push(&stmt);

        compile_module(&module, true);
    }

    #[test]
    fn test_nested_module_function() {
        // NOTE `children` is not appended to cause Rust sucks.
        let module = get_empty_module("core".to_string(), None);
        let another_module = get_empty_module("lib".to_string(), Some(&module));
        let mut last_module = get_empty_module("str".to_string(), Some(&another_module));

        // add function to module
        let sig = FunctionSignature {
            name: "turn_into".to_string(),
            return_type: Type::Primitive(Box::new(Primitive::Void)),
            inputs: Vec::new(),
            vis: metal_mir::types::visibility::Visibility::Public,
        };
        let def = FunctionDefinition {
            module: last_module.clone(),
            signature: sig.clone(),
            body: [
                Statement::Let(Box::new(Assignment {
                    name: "a_variable",
                    ty: Type::Primitive(Box::new(Primitive::U8)),
                    expr: Some(Expr::Add(Box::new(MathematicalValue {
                        a: Expr::Literal(Box::new(Literal::Number(Number {
                            primitive: Primitive::U8,
                            value: 5,
                        }))),
                        b: Expr::Literal(Box::new(Literal::Number(Number {
                            primitive: Primitive::U8,
                            value: 8,
                        }))),
                        signed: false,
                        float: false,
                        result_var_name: Some("a_variable"),
                    }))),
                })),
                Statement::Return(None),
            ]
            .to_vec(),
        };
        let stmt = Statement::FunctionDefine(Box::new(def));

        last_module.function_signatures.push(&sig);
        last_module.statements.push(&stmt);

        let compiled = String::from_utf8(compile_module(&last_module, true)).unwrap();
        assert_ne!(compiled, "".to_string());
    }
}
