// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

//! Metal library for compiling to LLVM IR using MIR.

use std::collections::HashMap;

use metal_mir::{
    parcel::Module,
    types::{visibility::Visibility, Type},
};

pub mod expr;
pub mod primitives;
pub mod stmt;
pub mod ty;

use llvm_sys::{
    prelude::{LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef},
    LLVMLinkage,
};

pub struct LLVMRefs {
    ctx: LLVMContextRef,
    builder: LLVMBuilderRef,
    module: LLVMModuleRef,
    locals: HashMap<&'static str, LLVMValueRef>,
}

pub trait CodeGenValue {
    fn codegen_value(&self, llvm: &mut LLVMRefs, module: &Module) -> LLVMValueRef;
}

pub trait CodeGenType {
    fn codegen_type(&self, llvm: &LLVMRefs, module: &Module) -> LLVMTypeRef;
}

pub fn get_types<'a>(
    llvm: &LLVMRefs,
    module: &Module,
    cap: &usize,
    types: impl IntoIterator<Item = &'a Type>,
) -> Vec<LLVMTypeRef> {
    let mut v = Vec::with_capacity(*cap);
    for t in types {
        v.push(t.codegen_type(llvm, module))
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
