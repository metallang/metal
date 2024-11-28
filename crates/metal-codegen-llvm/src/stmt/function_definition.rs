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
use crate::{core::get_module_full_name, get_linkage_from_vis};

impl CodeGenValue for FunctionDefinition<'_> {
    fn llvm_value(
        &self,
        llvm: &mut crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> LLVMValueRef {
        let fun_name = if self.signature.name != "main" {
            let module_name = get_module_full_name(module);
            module_name + "." + self.signature.name.as_str()
        } else {
            self.signature.name.clone()
        };

        let linkage = get_linkage_from_vis(&self.signature.vis);

        unsafe {
            let c_fun_name = CString::new(fun_name).unwrap();

            let function = LLVMAddFunction(
                llvm.module,
                c_fun_name.as_ptr(),
                self.signature.llvm_type(llvm, module),
            );
            LLVMSetLinkage(function, linkage);

            let entry_block = LLVMAppendBasicBlockInContext(llvm.ctx, function, c"entry".as_ptr());
            LLVMPositionBuilderAtEnd(llvm.builder, entry_block);

            llvm.locals.clear();

            for stmt in &self.body {
                stmt.llvm_value(llvm, module);
            }

            function
        }
    }
}
