use std::{ffi::CString, ptr};

use llvm_sys::{
    core::LLVMCreateMemoryBufferWithMemoryRangeCopy, ir_reader::LLVMParseIRInContext,
    linker::LLVMLinkModules2, prelude::*,
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

    /// # Safety
    /// The LLVM Pointer values here can cause crashes
    /// if set null or otherwise. Basically, only use
    /// if you know what you're doing.
    unsafe fn function(
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

    unsafe fn function(
        ident: metal_ast::Ident,
        ctx: LLVMContextRef,
        module: LLVMModuleRef,
        arguments: Vec<String>,
    ) -> Option<LLVMValueRef> {
        match ident.inner {
            "extern" => {
                if arguments.len() != 3 {
                    panic!("extern accepts exactly 3 arguments")
                }
                let name = &arguments[0];
                let ret_type = &arguments[1];
                let args = &arguments[2];

                let irstr = format!(
                    "
                    define {} @{}({}) unnamed_addr
                ",
                    ret_type, name, args
                );
                let ir = CString::new(irstr.clone()).unwrap();

                unsafe {
                    let irbuf = CString::new("ir_buffer").unwrap();
                    let ir_buffer = LLVMCreateMemoryBufferWithMemoryRangeCopy(
                        ir.as_ptr(),
                        irstr.len(),
                        irbuf.as_ptr(),
                    );

                    let mut error = ptr::null_mut();

                    let mut ptr = ptr::null_mut();

                    let suc = LLVMParseIRInContext(ctx, ir_buffer, &mut ptr, &mut error);

                    if suc != 0 {
                        panic!("{}", CString::from_raw(error).to_str().unwrap());
                    }

                    // TODO: diagnostics handling
                    let suc = LLVMLinkModules2(module, ptr);

                    if suc != 0 {
                        panic!("Failed to link LLVM modules in extern");
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
