// LLVM Codegen Library

use metal_ast::Primitives;
use std::collections::BTreeMap;
use llvm_sys::prelude::*;

// TODO: move this to another file?
struct Variable<'src> {
    pntr: LLVMValueRef,
    ty: metal_ast::Ty<'src>
}

pub struct CodeGen {
    module_name: &'static str,
    ctx: &inkwell::context::Context,
    builder: inkwell::builder::Builder,
    module: inkwell::module::Module
}

impl CodeGen {
    pub fn primitive(&self, primitive: Primitives) {
        match primitive {
            crate::Primitives::I8 => {
                self.ctx.i8_type()
            },
            crate::Primitives::I16 => {
                self.ctx.i16_type()
            },
            crate::Primitives::I32 => {
                self.ctx.i32_type()
            },
            crate::Primitives::I64 => {
                self.ctx.i64_type()
            },
            crate::Primitives::I128 => {
                self.ctx.i128_type()
            },

            crate::Primitives::F16 => {
                self.ctx.f16_type()
            },
            crate::Primitives::F32 => {
                self.ctx.f32_type()
            },
            crate::Primitives::F64 => {
                self.ctx.f64_type()
            },
            crate::Primitives::F128 => {
                self.ctx.f128_type()
            },

            crate::Primitives::String => {
                todo!()
            },
            crate::Primitives::Void => {
                self.ctx.void_type()
            }
        }
    }

    pub fn ty(&self, ty: metal_ast::Ty) -> inkwell::types::BasicType {
        match ty {
            Ty::Primitive(primitive) => {
                self.primitive(primitive);
            },
            _ => {
                todo!()
            }
        }
    }

    pub fn function_definition(
        &self,
        ty: metal_ast::FnDefStmt,
        library: bool,
    ) -> inkwell::values::FunctionValue<'ctx> {
        let ret_ty = ty.return_type
            .unwrap_or(metal_ast::Ty::Primitive(metal_ast::Primitives::Void));
        let return_type = self.ty(ret_ty);

        let mut params = Vec::with_capacity(ty.inputs.len());

        for fn_input in ty.inputs.iter() {
            params.push(self.ty(fn_input.ty))
        }

        // TODO: variadic argument support
        // like `*args`
        let function_type = return_type.fn_type(params.into(), false);
        let linkage = if ty.public {
            inkwell::module::Linkage::External
        } else {
            inkwell::module::Linkage::Internal
        };

        let fun_name = if library {
            let name = "";
            for module in self.module_name.split(".") {
                name += format!("{}{}", module.len(), module);
            }
            name + ty.ident.inner.len() + ty.ident.inner
        } else {
            // this only applies to
            // main.mt, the entry point of a program.
            ty.ident.inner
        };

        let function = self.module.add_function(&self.fun_name, function_type, Some(linkage));

        let start_block = self.ctx.append_basic_block(function, "start");
        self.builder.position_at_end(start_block);
        ty.body
    }

    pub fn expression(
        &self,
        expr: metal_ast::Expr,
        variables: &BTreeMap<String, Variable>
    ) {
        match expr {
            metal_ast::Expr::Number { ty, value } => {
                self.ty(ty)
            },
            metal_ast::Expr::Ident(ident) => {
                match variables.get(ident.inner) {
                    Some(v) => {
                        self.builder.build_load(v.ty, *v.pntr, ident.inner).unwrap()
                    },
                    None => {
                        panic!("If you see this, something broke  royally. The parser should prevent you from loading unknown variables!")
                    }
                }
            },
            metal_ast::Expr::FnCall { fn_name, arguments, module_name } => {
                let args = Vec::new();

                for inner_expr in arguments {
                    args.push(self.expression(inner_expr, variables));
                }

                self.builder.build_call(self.module.get_function(fn_name.inner), args.into(), fn_name.inner).unwrap()
            }
        }
    }
}
