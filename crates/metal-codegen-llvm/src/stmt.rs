use std::ffi::CString;

use llvm_sys::{
    core::{LLVMAddFunction, LLVMBuildAlloca, LLVMBuildStore},
    prelude::LLVMValueRef,
};
use metal_mir::stmt::Statement;

use super::{CodeGenType, CodeGenValue};
pub mod constant;
pub mod function_definition;

impl CodeGenValue for Statement {
    fn codegen_value(
        &self,
        llvm: &mut crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> LLVMValueRef {
        match self {
            Self::FunctionDefine(def) => def.codegen_value(llvm, module),
            Self::Constant(c) => c.codegen_value(llvm, module),
            Self::Let(l) => unsafe {
                let c_name = CString::new(l.name).unwrap();
                let a = LLVMBuildAlloca(
                    llvm.builder,
                    l.ty.codegen_type(llvm, module),
                    c_name.as_ptr(),
                );
                if let Some(e) = &l.expr {
                    LLVMBuildStore(llvm.builder, e.codegen_value(llvm, module), a);
                }
                llvm.locals.insert(l.name, a);
                a
            },
            Self::Extern(e) => unsafe {
                let c_name = CString::new(e.name.as_str()).unwrap();
                LLVMAddFunction(llvm.module, c_name.as_ptr(), e.codegen_type(llvm, module))
            },
        }
    }
}
