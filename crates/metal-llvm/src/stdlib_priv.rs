use llvm_sys::prelude::*;

// NOTE: private modules do NOT have submodules.
/// private modules stand at the core of the stdlib.
/// They allow us to define metal functionality in the stdlib
/// for compile time.
pub trait PrivateSTDLibModule {
    // private modules usually start with an underscore
    // But this name does not include that underscore.
    /// Private STDLib module name.
    /// i.e. `io`, `cmplib`, etc.
    fn libname() -> &'static str;

    fn functions() -> LLVMValueRef;
}

pub struct ExternalLib {
    ctx: LLVMContextRef,
    builder: LLVMBuilderRef,
    module: LLVMModuleRef,
}

impl PrivateSTDLibModule for ExternalLib {
    fn libname() -> &'static str {
        "pext"
    }

    fn functions() -> LLVMValueRef {
        todo!()
    }
}
