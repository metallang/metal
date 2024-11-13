use std::ffi::CString;

use llvm_sys::{
    core::{
        LLVMAddFunction, LLVMAppendBasicBlockInContext, LLVMPositionBuilderAtEnd, LLVMSetLinkage,
    },
    prelude::LLVMValueRef,
    LLVMLinkage,
};
use metal_mir::stmt::{functiondef::FunctionDefinition, Statement};

use super::{CodeGenType, CodeGenValue};

impl CodeGenValue for FunctionDefinition {
    fn codegen_value(
        &self,
        llvm: &crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> LLVMValueRef {
        // NOTE: cloned due to `self.signature.clone()` during LLVMAddFunction
        let fun_name = self.signature.name.clone();

        let linkage = match self.signature.vis {
            metal_mir::types::visibility::Visibility::Parcel => LLVMLinkage::LLVMInternalLinkage,
            metal_mir::types::visibility::Visibility::Private => LLVMLinkage::LLVMInternalLinkage,
            metal_mir::types::visibility::Visibility::Public => LLVMLinkage::LLVMExternalLinkage,
        };

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

            for stmt in &self.body {
                stmt.codegen_value(llvm, module);
            }

            function
        }
    }
}

impl CodeGenValue for Statement {
    fn codegen_value(
        &self,
        llvm: &crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> LLVMValueRef {
        match self {
            Self::FunctionDefine(def) => def.codegen_value(llvm, module),
        }
    }
}
