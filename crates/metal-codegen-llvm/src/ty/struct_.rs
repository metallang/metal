use llvm_sys::{core::LLVMStructTypeInContext, prelude::LLVMTypeRef};
use metal_mir::struct_::{Struct, StructField};

use super::{get_types, CodeGenType};

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
