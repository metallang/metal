// SPDX-License-Identifier: MIT
//! Compilation step for modules from MIR to LLVM IR

use std::{borrow::Cow, collections::HashMap, ffi::CString};

use llvm_sys::{
    analysis::LLVMVerifyModule,
    bit_writer,
    core::{LLVMPrintModuleToString, LLVMSetSourceFileName, LLVMSetTarget},
    prelude::LLVMTypeRef,
    target_machine::LLVMGetDefaultTargetTriple,
};
use metal_mir::{parcel::Module, struct_::Struct};

use crate::{
    safeties::{LLVMErrorMessage, MemoryBuffer},
    CodeGenType, CodeGenValue, LLVMRefs,
};

#[derive(Default)]
pub struct StructRepository {
    structs: HashMap<String, LLVMTypeRef>,
}

impl StructRepository {
    pub fn insert(&mut self, name: String, ty: LLVMTypeRef) {
        self.structs.insert(name, ty);
    }

    pub fn get(&self, name: &str) -> Option<LLVMTypeRef> {
        self.structs.get(name).copied()
    }

    pub fn get_or_insert(
        &mut self,
        name: String,
        struct_: &Struct,
        llvm: &mut LLVMRefs,
        module: &Module,
    ) -> LLVMTypeRef {
        if let Some(struct_) = self.get(&name) {
            struct_
        } else {
            let ty = struct_.llvm_type(llvm, module);
            self.insert(name, ty);
            ty
        }
    }
}

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

/// Compiles an LLVM module and returns either human-readable
/// LLVM IR or LLVM bytecode depending on `human_readable`.
pub fn compile_module(module: &Module, human_readable: bool, triple: &Option<String>) -> Vec<u8> {
    let mut llvm = LLVMRefs::new(module);

    let c_filename = CString::new(module.filename.as_str()).unwrap();

    unsafe { LLVMSetSourceFileName(llvm.module, c_filename.as_ptr(), c_filename.count_bytes()) };

    for stmt in &module.statements {
        stmt.llvm_value(&mut llvm, module);
    }

    unsafe {
        if let Some(triple) = triple {
            let string = CString::new(triple.as_str()).unwrap();
            LLVMSetTarget(llvm.module, string.as_ptr());
        } else {
            LLVMSetTarget(llvm.module, LLVMGetDefaultTargetTriple());
        }
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
    use std::collections::BTreeMap;

    use metal_mir::{
        expr::{
            literals::{Literal, Number},
            Assignment, Expr, MathematicalValue,
        },
        stmt::{constant::Constant, functiondef::FunctionDefinition, Statement},
        types::{function::FunctionSignature, primitives::Primitive, Type},
    };

    use super::*;

    fn get_empty_module(name: String) -> Module {
        Module {
            name: name.clone(),
            filename: name,
            statements: Vec::new(),
            imports: Vec::new(),
            library: false,
        }
    }

    #[test]
    fn test_empty_module() {
        let module = get_empty_module("empty".to_string());
        let compiled = String::from_utf8(compile_module(&module, true, &None)).unwrap();
        assert_ne!(compiled, "".to_string());
    }

    #[test]
    fn test_module_function() {
        let mut module = get_empty_module("testing".to_string());

        // add function to module
        let sig = FunctionSignature {
            name: "main".to_string(),
            return_type: Type::Primitive(Box::new(Primitive::Void)),
            inputs: BTreeMap::new(),
            vis: metal_mir::types::visibility::Visibility::Public,
        };
        let def = FunctionDefinition {
            signature: sig.clone(),
            body: [
                Statement::Let(Box::new(Assignment {
                    name: "a_variable".to_string(),
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
                        result_var_name: Some("a_variable".to_string()),
                    }))),
                })),
                Statement::Return(None),
            ]
            .to_vec(),
        };
        let stmt = Statement::FunctionDefine(Box::new(def));

        module.statements.push(Box::new(stmt));

        let compiled = String::from_utf8(compile_module(&module, true, &None)).unwrap();
        assert_ne!(compiled, "".to_string());
    }

    #[test]
    #[should_panic]
    fn test_panic_global_var_expr() {
        let mut module = get_empty_module("testing".to_string());
        let global_var = Constant {
            name: "some_constant".to_string(),
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
                result_var_name: Some("some_constant".to_string()),
            })),
            ty: Primitive::I32,
            vis: metal_mir::types::visibility::Visibility::Private,
        };
        let stmt = Statement::Constant(Box::new(global_var));

        module.statements.push(Box::new(stmt));

        compile_module(&module, true, &None);
    }

    #[test]
    fn test_nested_module_function() {
        // NOTE `children` is not appended as to avoid lengthy duplication
        // of modules as this test doesn't use the children
        let mut module = get_empty_module("core.string".to_string());

        // add function to module
        let sig = FunctionSignature {
            name: "turn_into".to_string(),
            return_type: Type::Primitive(Box::new(Primitive::Void)),
            inputs: BTreeMap::new(),
            vis: metal_mir::types::visibility::Visibility::Public,
        };
        let def = FunctionDefinition {
            signature: sig.clone(),
            body: [
                Statement::Let(Box::new(Assignment {
                    name: "a_variable".to_string(),
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
                        result_var_name: Some("a_variable".to_string()),
                    }))),
                })),
                Statement::Return(None),
            ]
            .to_vec(),
        };
        let stmt = Statement::FunctionDefine(Box::new(def));

        module.statements.push(Box::new(stmt));

        let compiled = String::from_utf8(compile_module(&module, true, &None)).unwrap();
        assert_ne!(compiled, "".to_string());
    }
}
