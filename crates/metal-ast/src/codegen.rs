use metal_llvm::{CodeGen, inkwell};
use super::{FnDefStmt, Ty};

impl CodeGen for Ty {
    fn codegen(
        &self,
        ctx: &inkwell::context::Context,
        builder: &inkwell::builder::Builder,
        module: &inkwell::module::Module
    ) {
        match self {
            Ty::Primitive(primitive) => {
                match primitive {
                    crate::Primitives::I8 => {
                        ctx.i8_type()
                    },
                    crate::Primitives::I16 => {
                        ctx.i16_type()
                    },
                    crate::Primitives::I32 => {
                        ctx.i32_type()
                    },
                    crate::Primitives::I64 => {
                        ctx.i64_type()
                    },
                    crate::Primitives::I128 => {
                        ctx.i128_type()
                    },

                    crate::Primitives::F16 => {
                        ctx.f16_type()
                    },
                    crate::Primitives::F32 => {
                        ctx.f32_type()
                    },
                    crate::Primitives::F64 => {
                        ctx.f64_type()
                    },
                    crate::Primitives::F128 => {
                        ctx.f128_type()
                    },

                    crate::Primitives::String => {
                        todo!()
                    },
                    crate::Primitives::Void => {
                        ctx.void_type()
                    }
                }
            },
            _ => {
                todo!()
            }
        }
    }
}

impl CodeGen for FnDefStmt {
    fn codegen(
        &self,
        ctx: &inkwell::context::Context,
        builder: &inkwell::builder::Builder,
        module: &inkwell::module::Module
    ) {
        
    }
}
