// SPDX-License-Identifier: MIT

use std::ffi::CString;

use llvm_sys::{
    core::{
        LLVMAddFunction, LLVMAppendBasicBlockInContext, LLVMPositionBuilderAtEnd, LLVMSetLinkage,
    },
    prelude::LLVMValueRef,
};
use metal_mir::stmt::functiondef::FunctionDefinition;

use super::{CodeGenType, CodeGenValue};
use crate::get_linkage_from_vis;

impl CodeGenValue for FunctionDefinition {
    fn codegen_value(
        &self,
        llvm: &mut crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> LLVMValueRef {
        let fun_name = self.signature.name.as_str();

        let linkage = get_linkage_from_vis(&self.signature.vis);

        unsafe {
            let c_fun_name = CString::new(fun_name).unwrap();

            let function = LLVMAddFunction(
                llvm.module,
                c_fun_name.as_ptr(),
                self.signature.codegen_type(llvm, module),
            );
            LLVMSetLinkage(function, linkage);

            let entry_block = LLVMAppendBasicBlockInContext(llvm.ctx, function, c"entry".as_ptr());
            LLVMPositionBuilderAtEnd(llvm.builder, entry_block);

            llvm.locals.clear();

            for stmt in &self.body {
                stmt.codegen_value(llvm, module);
            }

            function
        }
    }
}
