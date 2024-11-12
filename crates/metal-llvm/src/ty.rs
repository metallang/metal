use llvm_sys::core::{LLVMArrayType2, LLVMFunctionType, LLVMStructTypeInContext};
use metal_mir::types::Type;

use super::{get_types, CodeGenType};

impl CodeGenType for Type {
    fn codegen_type(
        &self,
        llvm: &crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> llvm_sys::prelude::LLVMTypeRef {
        match self {
            Self::Primitive(p) => p.codegen_type(llvm, module),
            Self::Composite(c) => unsafe {
                match *c.clone() {
                    metal_mir::types::Composite::Tuple(t) => {
                        let num_types = t.types.len();
                        LLVMStructTypeInContext(
                            llvm.ctx,
                            get_types(llvm, module, t.types).as_mut_ptr(),
                            num_types.try_into().unwrap(),
                            0,
                        )
                    }
                    metal_mir::types::Composite::Array(a) => {
                        LLVMArrayType2(a.item_type.codegen_type(llvm, module), a.size)
                    }
                }
            },
            Self::Function(f) => unsafe {
                LLVMFunctionType(
                    f.return_type.codegen_type(llvm, module),
                    get_types(llvm, module, f.arguments.clone()).as_mut_ptr(),
                    f.arguments.len().try_into().unwrap(),
                    0,
                )
            },
        }
    }
}
