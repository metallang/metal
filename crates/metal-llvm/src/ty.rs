use llvm_sys::{
    core::{LLVMArrayType2, LLVMFunctionType, LLVMStructTypeInContext},
    prelude::LLVMTypeRef,
};
use metal_mir::{
    struct_::{Struct, StructField},
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
            let len = self.inputs.len();

            let mut types_to_convert = get_types(llvm, module, &len, &self.inputs);
            LLVMFunctionType(
                self.return_type.codegen_type(llvm, module),
                types_to_convert.as_mut_ptr(),
                len.try_into().unwrap(),
                0,
            )
        }
    }
}

fn get_types_struct(
    llvm: &crate::LLVMRefs,
    module: &metal_mir::parcel::Module,
    fields: &Vec<StructField>,
) -> (usize, Vec<LLVMTypeRef>) {
    let num_props = fields.len();

    let mut types_to_convert = Vec::with_capacity(num_props);
    for prop in fields {
        types_to_convert.push(&prop.ty);
    }

    (
        num_props,
        get_types(llvm, module, &num_props, types_to_convert),
    )
}

impl CodeGenType for Struct {
    fn codegen_type(
        &self,
        llvm: &crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> llvm_sys::prelude::LLVMTypeRef {
        let (num_props, mut types) = get_types_struct(llvm, module, &self.fields);

        unsafe {
            LLVMStructTypeInContext(
                llvm.ctx,
                types.as_mut_ptr(),
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
