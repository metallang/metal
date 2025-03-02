// SPDX-License-Identifier: MIT

use llvm_sys::core::{LLVMArrayType2, LLVMStructTypeInContext};
use metal_mir::types::Type;

use super::{get_types, CodeGenType};

pub mod function_signature;
pub mod struct_;

impl CodeGenType for Type {
    fn llvm_type(
        &self,
        llvm: &mut crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> llvm_sys::prelude::LLVMTypeRef {
        match self {
            Self::Primitive(p) => p.llvm_type(llvm, module),
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
                        LLVMArrayType2(a.item_type.llvm_type(llvm, module), a.size)
                    }
                }
            },
            Self::Function(f) => f.llvm_type(llvm, module),
            Self::Struct(s) => s.llvm_type(llvm, module),
            Self::Unknown => panic!("Unknown types cannot be compiled"),
        }
    }
}
