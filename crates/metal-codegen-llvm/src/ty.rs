// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

use llvm_sys::core::{LLVMArrayType2, LLVMStructTypeInContext};
use metal_mir::types::Type;

use super::{get_types, CodeGenType};

pub mod function_signature;
pub mod struct_;

impl CodeGenType for Type {
    fn codegen_type(
        &self,
        llvm: &crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> llvm_sys::prelude::LLVMTypeRef {
        match self {
            Self::Primitive(p) => p.codegen_type(llvm, module),
            Self::Composite(c) => unsafe {
                match c.as_ref() {
                    metal_mir::types::Composite::Tuple(t) => {
                        let num_types = t.types.len();
                        LLVMStructTypeInContext(
                            llvm.ctx,
                            get_types(llvm, module, &num_types, &t.types).as_mut_ptr(),
                            num_types.try_into().unwrap(),
                            0,
                        )
                    }
                    metal_mir::types::Composite::Array(a) => {
                        LLVMArrayType2(a.item_type.codegen_type(llvm, module), a.size)
                    }
                }
            },
            Self::Function(f) => f.codegen_type(llvm, module),
            Self::Struct(s) => s.codegen_type(llvm, module),
        }
    }
}
