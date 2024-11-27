// SPDX-License-Identifier: MIT
//! Compilation step for modules from MIR to LLVM IR

use std::ffi::CString;

use llvm_sys::{bit_writer, core::LLVMPrintModuleToString};
use metal_mir::parcel::Module;

use crate::{safeties::MemoryBuffer, CodeGenValue, LLVMRefs};

/// Compiles an LLVM module and returns either human-readable
/// LLVM IR or LLVM bytecode depending on `human_readable`.
pub fn compile_module(module: &Module, human_readable: bool) -> Vec<u8> {
    let mut llvm = LLVMRefs::new(module);

    for stmt in &module.statements {
        stmt.llvm_value(&mut llvm, module);
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
            Assignment, Expr, Load, MathematicalValue,
        },
        stmt::{constant::Constant, functiondef::FunctionDefinition, return_::Return, Statement},
        types::{function::FunctionSignature, primitives::Primitive, Type},
    };

    use super::*;

    fn get_empty_module() -> Module {
        Module {
            name: "testmodule".to_string(),
            parent: Option::None,
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
        let module = get_empty_module();
        let compiled = String::from_utf8(compile_module(&module, true)).unwrap();
        assert_ne!(compiled, "".to_string());
        println!("compiled module: \n\n{}", compiled);
    }

    #[test]
    fn test_module_function() {
        let mut module = get_empty_module();

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
                Statement::Return(Box::new(Return(Expr::Load(Box::new(Load {
                    name: "a_variable",
                    ty: Type::Primitive(Box::new(Primitive::U8)),
                }))))),
            ]
            .to_vec(),
        };

        module.function_signatures.push(sig);
        module
            .statements
            .push(Statement::FunctionDefine(Box::new(def)));

        let compiled = String::from_utf8(compile_module(&module, true)).unwrap();
        assert_ne!(compiled, "".to_string());
        println!("compiled module: \n\n{}", compiled);
    }

    #[test]
    #[should_panic]
    fn test_panic_global_var_expr() {
        let mut module = get_empty_module();
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

        module
            .statements
            .push(Statement::Constant(Box::new(global_var)));

        compile_module(&module, true);
    }
}
