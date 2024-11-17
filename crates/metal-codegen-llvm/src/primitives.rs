// SPDX-License-Identifier: MIT

use llvm_sys::core::{
    LLVMArrayType2, LLVMDoubleTypeInContext, LLVMFP128TypeInContext, LLVMFloatTypeInContext,
    LLVMIntTypeInContext, LLVMVoidTypeInContext,
};
use metal_mir::types::primitives::Primitive;

use crate::CodeGenType;

impl CodeGenType for Primitive {
    fn codegen_type(
        &self,
        llvm: &crate::LLVMRefs,
        _module: &metal_mir::parcel::Module,
    ) -> llvm_sys::prelude::LLVMTypeRef {
        unsafe {
            match self {
                // Integers (signed)
                Self::I8 | Self::U8 => LLVMIntTypeInContext(llvm.ctx, 8),
                Self::I16 | Self::U16 => LLVMIntTypeInContext(llvm.ctx, 16),
                Self::I32 | Self::U32 => LLVMIntTypeInContext(llvm.ctx, 32),
                Self::I64 | Self::U64 => LLVMIntTypeInContext(llvm.ctx, 64),
                Self::I128 | Self::U128 => LLVMIntTypeInContext(llvm.ctx, 128),

                // Floats; TODO: support f16, maybe f80 too?
                Self::F32 => LLVMFloatTypeInContext(llvm.ctx),
                Self::F64 => LLVMDoubleTypeInContext(llvm.ctx),
                Self::F128 => LLVMFP128TypeInContext(llvm.ctx),

                // Strings
                Self::Literal(length) => {
                    let char_ty = LLVMIntTypeInContext(llvm.ctx, 8);
                    LLVMArrayType2(char_ty, *length)
                }

                // NOTE: Void is represented by an empty tuple. `()`
                // Void
                Self::Void => LLVMVoidTypeInContext(llvm.ctx),
            }
        }
    }
}
