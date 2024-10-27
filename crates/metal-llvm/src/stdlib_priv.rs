use std::{ffi::CString, ptr};

use llvm_sys::{
    core::LLVMCreateMemoryBufferWithMemoryRangeCopy, ir_reader::LLVMParseIRInContext, prelude::*,
};

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

    fn function(
        ident: metal_ast::Ident,
        ctx: LLVMContextRef,
        module: LLVMModuleRef,
        arguments: Vec<String>,
    ) -> Option<LLVMValueRef>;
}

pub struct ExternalLib;

impl PrivateSTDLibModule for ExternalLib {
    fn libname() -> &'static str {
        "pext"
    }

    fn function(
        ident: metal_ast::Ident,
        ctx: LLVMContextRef,
        module: LLVMModuleRef,
        arguments: Vec<String>,
    ) -> Option<LLVMValueRef> {
        match ident.inner {
            "extern" => {
                if arguments.len() != 3 {
                    panic!("extern does not accept more or less than 3 arguments")
                }
                let name = &arguments[0];
                let ret_type = &arguments[1];
                let args = &arguments[2];

                let ir = format!(
                    "
                    define {} @{}({}) unnamed_addr
                ",
                    ret_type, name, args
                );

                unsafe {
                    let ir_buffer = LLVMCreateMemoryBufferWithMemoryRangeCopy(
                        ir.as_ptr() as *const i8,
                        ir.len(),
                        "ir_buffer".as_ptr() as *const i8,
                    );

                    let mut error = ptr::null_mut();

                    let suc =
                        LLVMParseIRInContext(ctx, ir_buffer, [module].as_mut_ptr(), &mut error);

                    if suc != 0 {
                        panic!("{}", CString::from_raw(error).to_str().unwrap());
                    }

                    None
                }
            }
            // TODO: proper error handling
            _ => panic!(
                "{}",
                format!("ExternalLib does not include the {} function", ident.inner)
            ),
        }
    }
}
