// SPDX-License-Identifier: MIT

//! Metal library for compiling to LLVM IR using MIR.

use std::{collections::HashMap, ffi::CString};

use metal_mir::{
    parcel::Module,
    types::{visibility::Visibility, Type},
};

pub mod core;
pub mod expr;
pub mod primitives;
pub mod safeties;
pub mod stmt;
pub mod ty;

use llvm_sys::{
    core::{
        LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilder, LLVMDisposeBuilder,
        LLVMDisposeModule, LLVMModuleCreateWithNameInContext,
    },
    prelude::{LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef},
    LLVMLinkage,
};

pub struct LLVMRefs {
    ctx: LLVMContextRef,
    builder: LLVMBuilderRef,
    module: LLVMModuleRef,
    locals: HashMap<&'static str, LLVMValueRef>,
}

impl LLVMRefs {
    pub fn new(module: &Module) -> Self {
        let ctx = unsafe { LLVMContextCreate() };
        let module_name = CString::new(module.name.as_str()).unwrap();
        unsafe {
            Self {
                ctx,
                builder: LLVMCreateBuilder(),
                module: LLVMModuleCreateWithNameInContext(module_name.as_ptr(), ctx),
                locals: HashMap::new(),
            }
        }
    }
}

impl Drop for LLVMRefs {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
            LLVMDisposeModule(self.module);
            LLVMContextDispose(self.ctx);
        }
    }
}

pub trait CodeGenValue {
    fn llvm_value(&self, llvm: &mut LLVMRefs, module: &Module) -> LLVMValueRef;
}

pub trait CodeGenType {
    fn llvm_type(&self, llvm: &LLVMRefs, module: &Module) -> LLVMTypeRef;
}

pub fn get_types<'a>(
    llvm: &LLVMRefs,
    module: &Module,
    cap: &usize,
    types: impl IntoIterator<Item = &'a Type>,
) -> Vec<LLVMTypeRef> {
    let mut v = Vec::with_capacity(*cap);
    for t in types {
        v.push(t.llvm_type(llvm, module))
    }
    v
}

pub fn get_linkage_from_vis(visibility: &Visibility) -> LLVMLinkage {
    match visibility {
        Visibility::Parcel => LLVMLinkage::LLVMInternalLinkage,
        Visibility::Private => LLVMLinkage::LLVMExternalLinkage,
        Visibility::Public => LLVMLinkage::LLVMExternalLinkage,
    }
}
