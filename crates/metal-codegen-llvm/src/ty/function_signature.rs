// SPDX-License-Identifier: MIT

use llvm_sys::core::LLVMFunctionType;
use metal_mir::types::function::FunctionSignature;

use super::{get_types, CodeGenType};

impl CodeGenType for FunctionSignature {
    fn llvm_type(
        &self,
        llvm: &mut crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> llvm_sys::prelude::LLVMTypeRef {
        unsafe {
            let len = self.inputs.len();

            let mut types_to_convert = get_types(llvm, module, &len, self.inputs.values());
            LLVMFunctionType(
                self.return_type.llvm_type(llvm, module),
                types_to_convert.as_mut_ptr(),
                len.try_into().unwrap(),
                0,
            )
        }
    }
}
