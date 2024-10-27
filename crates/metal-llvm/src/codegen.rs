// LLVM Codegen Library

use std::{collections::BTreeMap, ffi::CStr};

use llvm_sys::{
    core::{
        LLVMAddFunction, LLVMAppendBasicBlockInContext, LLVMBuildCall2, LLVMBuildLoad2,
        LLVMConstInt, LLVMDoubleTypeInContext, LLVMFP128TypeInContext, LLVMFloatTypeInContext,
        LLVMFunctionType, LLVMGetElementType, LLVMGetNamedFunction, LLVMGetReturnType,
        LLVMIntTypeInContext, LLVMPositionBuilderAtEnd, LLVMPrintModuleToString, LLVMSetLinkage,
        LLVMTypeOf, LLVMVoidTypeInContext,
    },
    prelude::*,
    LLVMLinkage,
};

// TODO: move this to another file?
pub struct Variable<'src> {
    pntr: LLVMValueRef,
    ty: metal_ast::Ty<'src>,
}

pub struct CodeGen {
    module_name: &'static str,
    ctx: LLVMContextRef,
    builder: LLVMBuilderRef,
    module: LLVMModuleRef,
}

impl CodeGen {
    // complete codegen process
    pub fn finish(&self) -> &'static str {
        unsafe {
            let ir = LLVMPrintModuleToString(self.module);
            let cir = CStr::from_ptr(ir);
            // TODO: specify which module specifically.
            let string_ir = cir
                .to_str()
                .expect("Failed to turn LLVM module into string");
            string_ir
        }
    }

    pub fn primitive(&self, primitive: &metal_ast::Primitives) -> LLVMTypeRef {
        match primitive {
            metal_ast::Primitives::I8 => unsafe { LLVMIntTypeInContext(self.ctx, 8) },
            metal_ast::Primitives::I16 => unsafe { LLVMIntTypeInContext(self.ctx, 16) },
            metal_ast::Primitives::I32 => unsafe { LLVMIntTypeInContext(self.ctx, 32) },
            metal_ast::Primitives::I64 => unsafe { LLVMIntTypeInContext(self.ctx, 64) },
            metal_ast::Primitives::I128 => unsafe { LLVMIntTypeInContext(self.ctx, 128) },

            metal_ast::Primitives::F32 => unsafe { LLVMFloatTypeInContext(self.ctx) },
            metal_ast::Primitives::F64 => unsafe { LLVMDoubleTypeInContext(self.ctx) },
            metal_ast::Primitives::F128 => unsafe { LLVMFP128TypeInContext(self.ctx) },

            metal_ast::Primitives::String => {
                todo!()
            }
            metal_ast::Primitives::Void => unsafe { LLVMVoidTypeInContext(self.ctx) },
        }
    }

    pub fn ty(&self, ty: &metal_ast::Ty) -> LLVMTypeRef {
        match ty {
            metal_ast::Ty::Primitive(primitive) => self.primitive(primitive),
            _ => {
                todo!()
            }
        }
    }

    pub fn function_definition(&self, ty: metal_ast::FnDefStmt, library: bool) -> LLVMValueRef {
        let ret_ty = ty
            .return_type
            .unwrap_or(metal_ast::Ty::Primitive(metal_ast::Primitives::Void));
        let return_type = self.ty(&ret_ty);

        let mut params = Vec::with_capacity(ty.inputs.len());

        for fn_input in ty.inputs {
            params.push(self.ty(&fn_input.ty))
        }

        let linkage = if ty.public {
            LLVMLinkage::LLVMExternalLinkage
        } else {
            LLVMLinkage::LLVMInternalLinkage
        };

        let fun_name: String = if library {
            let mut name = "".to_string();
            for module in self.module_name.split('.') {
                let m1 = module.len().to_string() + module;
                let m = m1.as_str();
                name += m;
            }
            let n = name + ty.ident.inner.len().to_string().as_str() + ty.ident.inner;
            n
        } else {
            // this only applies to
            // main.mt, the entry point of a program.
            ty.ident.inner.to_string()
        };

        unsafe {
            // TODO: variadic argument support
            // like `*args`
            let function = LLVMAddFunction(
                self.module,
                fun_name.as_ptr() as *const i8,
                LLVMFunctionType(return_type, params.as_mut_ptr(), params.len() as u32, 0),
            );
            LLVMSetLinkage(function, linkage);

            let entry_block =
                LLVMAppendBasicBlockInContext(self.ctx, function, "entry".as_ptr() as *const i8);
            LLVMPositionBuilderAtEnd(self.builder, entry_block);

            // TODO: global variables
            let variables = BTreeMap::new();

            for stmt in ty.body {
                match stmt {
                    // These aren't allowed inside of function bodies
                    metal_ast::Statement::ClassDef(_) => todo!(),
                    metal_ast::Statement::FnDef(_) => todo!(),
                    metal_ast::Statement::Import(_) => panic!(),

                    metal_ast::Statement::Expr(expr) => {
                        self.expression(expr.expr, &variables);
                    }
                }
            }

            function
        }
    }

    pub fn expression(
        &self,
        expr: metal_ast::Expr,
        variables: &BTreeMap<String, Variable>,
    ) -> LLVMValueRef {
        match expr {
            metal_ast::Expr::Number { ty, value } => unsafe {
                LLVMConstInt(self.ty(&ty), value, 1)
            },
            metal_ast::Expr::Ident(ident) => match variables.get(ident.inner) {
                Some(v) => unsafe {
                    LLVMBuildLoad2(
                        self.builder,
                        self.ty(&v.ty),
                        v.pntr,
                        ident.inner.as_ptr() as *const i8,
                    )
                },
                None => {
                    panic!("If you see this, something broke  royally. The parser should prevent you from loading unknown variables!")
                }
            },
            metal_ast::Expr::FnCall {
                fn_name,
                arguments,
                module_name: _,
            } => {
                let mut args = Vec::new();

                for inner_expr in arguments {
                    args.push(self.expression(inner_expr, variables));
                }

                unsafe {
                    let c_fn_name = fn_name.inner.as_ptr() as *const i8;
                    // TODO: I don't know how to error-handle LLVM lol
                    let func = LLVMGetNamedFunction(self.module, c_fn_name);
                    let func_ty = LLVMGetElementType(LLVMTypeOf(func));
                    LLVMBuildCall2(
                        self.builder,
                        LLVMGetReturnType(func_ty),
                        func,
                        args.as_mut_ptr(),
                        args.len() as u32,
                        c_fn_name,
                    )
                }
            }
        }
    }
}
