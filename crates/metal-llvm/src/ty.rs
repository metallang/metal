use llvm_sys::core::{LLVMArrayType2, LLVMFunctionType, LLVMStructTypeInContext};
use metal_mir::{
    structure::Struct,
    types::{function::FunctionSignature, Type},
};

use super::{get_types, CodeGenType};

impl CodeGenType for FunctionSignature {
    fn codegen_type(
        &self,
        llvm: &crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> llvm_sys::prelude::LLVMTypeRef {
        unsafe {
            let len = self.arguments.len();
            LLVMFunctionType(
                self.return_type.codegen_type(llvm, module),
                get_types(llvm, module, &self.arguments).as_mut_ptr(),
                len.try_into().unwrap(),
                0,
            )
        }
    }
}

impl CodeGenType for Struct {
    /// ## Panics
    /// panics if `properties` doesn't exist, meaning
    /// this struct doesn't have real representation.
    fn codegen_type(
        &self,
        llvm: &crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> llvm_sys::prelude::LLVMTypeRef {
        let properties = self.properties.clone().unwrap();
        let num_props = properties.len();

        let mut types_to_convert = Vec::with_capacity(num_props);
        for prop in properties {
            types_to_convert.push(prop.ty)
        }

        unsafe {
            LLVMStructTypeInContext(
                llvm.ctx,
                get_types(llvm, module, &types_to_convert).as_mut_ptr(),
                num_props.try_into().unwrap(),
                0,
            )
        }
    }
}

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
                            get_types(llvm, module, &t.types).as_mut_ptr(),
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
