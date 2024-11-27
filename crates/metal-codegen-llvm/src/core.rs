// SPDX-License-Identifier: MIT
//! Compilation step for modules from MIR to LLVM IR

use std::ffi::CString;

use llvm_sys::{bit_writer, core::LLVMPrintModuleToString};
use metal_mir::parcel::Module;

use crate::{safeties::MemoryBuffer, CodeGenValue, LLVMRefs};

/// Compiles an LLVM module and returns either human-readable
/// LLVM IR or LLVM bytecode depending on `human_readable`.
#[inline]
pub fn compile_module(module: &Module, human_readable: bool) -> Vec<u8> {
    let mut llvm = LLVMRefs::new(module);

    for stmt in &module.statements {
        stmt.llvm_value(&mut llvm, module);
    }

    if human_readable {
        let module_ir = unsafe { CString::from_raw(LLVMPrintModuleToString(llvm.module)) };
        module_ir.into_bytes()
    } else {
        let buf = unsafe { bit_writer::LLVMWriteBitcodeToMemoryBuffer(llvm.module) };
        MemoryBuffer::new(buf).to_vec()
    }
}
