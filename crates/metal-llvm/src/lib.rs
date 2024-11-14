//! Metal library for compiling to LLVM IR using MIR.

use metal_mir::parcel::Module;

pub mod expr;
pub mod primitives;
pub mod stmt;
pub mod ty;

use llvm_sys::prelude::{LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef};

pub struct LLVMRefs {
    ctx: LLVMContextRef,
    builder: LLVMBuilderRef,
    module: LLVMModuleRef,
}

pub trait CodeGenValue {
    fn codegen_value(&self, llvm: &LLVMRefs, module: &Module) -> LLVMValueRef;
}

pub trait CodeGenType {
    fn codegen_type(&self, llvm: &LLVMRefs, module: &Module) -> LLVMTypeRef;
}

pub fn get_types(
    llvm: &LLVMRefs,
    module: &Module,
    types: &[metal_mir::types::Type],
) -> Vec<LLVMTypeRef> {
    let mut v = Vec::with_capacity(types.len());
    for t in types {
        v.push(t.codegen_type(llvm, module))
    }
    v
}
