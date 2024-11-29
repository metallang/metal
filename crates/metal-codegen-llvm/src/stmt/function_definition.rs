// SPDX-License-Identifier: MIT

use std::ffi::CString;

use llvm_sys::{
    core::{
        LLVMAddFunction, LLVMAppendBasicBlockInContext, LLVMGetParam, LLVMPositionBuilderAtEnd, LLVMSetLinkage, LLVMSetValueName2
    },
    prelude::LLVMValueRef,
};
use metal_mir::stmt::functiondef::FunctionDefinition;

use super::{CodeGenType, CodeGenValue};
use crate::{core::get_module_full_name, get_linkage_from_vis};

impl CodeGenValue for FunctionDefinition {
    fn llvm_value(
        &self,
        llvm: &mut crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> LLVMValueRef {
        let fun_name = if self.signature.name != "main" {
            let module_name = get_module_full_name(module.to_owned());
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

            let mut idx = 0;

            for (input_name, _) in self.signature.inputs.iter() {
                let param = LLVMGetParam(function, idx);
                idx += 1;
                let c_name = CString::new(input_name.as_str()).unwrap();
                LLVMSetValueName2(param, c_name.as_ptr(), c_name.count_bytes());
                llvm.locals.insert(input_name.to_string(), param);
            };

            for stmt in &self.body {
                stmt.llvm_value(llvm, module);
            }

            function
        }
    }
}
