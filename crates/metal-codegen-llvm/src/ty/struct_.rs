// SPDX-License-Identifier: MIT

use std::ffi::CString;

use llvm_sys::{
    core::{LLVMStructCreateNamed, LLVMStructSetBody},
    prelude::LLVMTypeRef,
};
use metal_mir::struct_::{Struct, StructField};

use super::{get_types, CodeGenType};

fn get_types_struct(
    llvm: &mut crate::LLVMRefs,
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
    fn llvm_type(
        &self,
        llvm: &mut crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> llvm_sys::prelude::LLVMTypeRef {
        let full_name = module.name.clone() + "." + self.name.as_str();
        if let Some(s) = llvm.struct_repo.get(&full_name) {
            return s;
        }

        let (_, mut element_types) = get_types_struct(llvm, module, &self.fields);
        let name = CString::new(full_name.as_str()).unwrap();

        let strct = unsafe { LLVMStructCreateNamed(llvm.ctx, name.as_ptr()) };
        unsafe {
            LLVMStructSetBody(
                strct,
                element_types.as_mut_ptr(),
                self.fields.len().try_into().unwrap(),
                0,
            )
        };

        llvm.struct_repo.insert(full_name, strct);

        strct
    }
}
