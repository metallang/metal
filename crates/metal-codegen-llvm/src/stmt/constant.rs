// SPDX-License-Identifier: MIT

use std::ffi::CString;

use llvm_sys::{
    core::{LLVMAddGlobal, LLVMSetInitializer, LLVMSetLinkage},
    prelude::LLVMValueRef,
};
use metal_mir::{expr::Expr, stmt::constant::Constant};

use super::{CodeGenType, CodeGenValue};
use crate::get_linkage_from_vis;

impl CodeGenValue for Constant {
    fn llvm_value(
        &self,
        llvm: &mut crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> LLVMValueRef {
        let cname = CString::new(self.name.as_str()).unwrap();

        unsafe {
            let global_var =
                LLVMAddGlobal(llvm.module, self.ty.llvm_type(llvm, module), cname.as_ptr());

            match self.expr {
                Expr::Literal(_) => {
                    let val = self.expr.llvm_value(llvm, module);
                    LLVMSetInitializer(global_var, val);
                }
                _ => panic!("Expression is unsupported for use as a global variable"),
            }

            let linkage = get_linkage_from_vis(&self.vis);
            LLVMSetLinkage(global_var, linkage);

            global_var
        }
    }
}
