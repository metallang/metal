use std::ffi::CString;

use llvm_sys::{
    core::{
        LLVMAddFunction, LLVMAddGlobal, LLVMAppendBasicBlockInContext, LLVMBuildAlloca,
        LLVMPositionBuilderAtEnd, LLVMSetInitializer, LLVMSetLinkage,
    },
    prelude::LLVMValueRef,
};
use metal_mir::{
    expr::Expr,
    stmt::{constant::Constant, functiondef::FunctionDefinition, Statement},
};

use super::{CodeGenType, CodeGenValue};
use crate::get_linkage_from_vis;

impl CodeGenValue for FunctionDefinition {
    fn codegen_value(
        &self,
        llvm: &mut crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> LLVMValueRef {
        let fun_name = self.signature.name.as_str();

        let linkage = get_linkage_from_vis(&self.signature.vis);

        unsafe {
            let c_fun_name = CString::new(fun_name).unwrap();

            let function = LLVMAddFunction(
                llvm.module,
                c_fun_name.as_ptr(),
                self.signature.codegen_type(llvm, module),
            );
            LLVMSetLinkage(function, linkage);

            let entry_block = LLVMAppendBasicBlockInContext(llvm.ctx, function, c"entry".as_ptr());
            LLVMPositionBuilderAtEnd(llvm.builder, entry_block);

            llvm.locals.clear();

            for stmt in &self.body {
                stmt.codegen_value(llvm, module);
            }

            function
        }
    }
}

impl CodeGenValue for Constant {
    fn codegen_value(
        &self,
        llvm: &mut crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> LLVMValueRef {
        let cname = CString::new(self.name).unwrap();

        unsafe {
            let global_var = LLVMAddGlobal(
                llvm.module,
                self.ty.codegen_type(llvm, module),
                cname.as_ptr(),
            );

            match self.expr {
                Expr::Literal(_) => {
                    let val = self.expr.codegen_value(llvm, module);
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

impl CodeGenValue for Statement {
    fn codegen_value(
        &self,
        llvm: &mut crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> LLVMValueRef {
        match self {
            Self::FunctionDefine(def) => def.codegen_value(llvm, module),
            Self::Constant(c) => c.codegen_value(llvm, module),
            Self::Let(l) => unsafe {
                let c_name = CString::new(l.name).unwrap();
                let a = LLVMBuildAlloca(
                    llvm.builder,
                    l.ty.codegen_type(llvm, module),
                    c_name.as_ptr(),
                );
                llvm.locals.insert(l.name, a);
                a
            },
            Self::Extern(e) => unsafe {
                let c_name = CString::new(e.name).unwrap();
                LLVMAddFunction(
                    llvm.module,
                    c_name.as_ptr(),
                    e.sig.codegen_type(llvm, module),
                )
            },
        }
    }
}
