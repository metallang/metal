// SPDX-License-Identifier: MIT
//! Safe Rust abstractions for several LLVM types

use std::slice;

use llvm_sys::{
    core::{LLVMDisposeMemoryBuffer, LLVMGetBufferSize, LLVMGetBufferStart},
    prelude::LLVMMemoryBufferRef,
};

/// Safe abstraction over LLVM's MemoryBuffer
pub struct MemoryBuffer {
    llvm_buf: LLVMMemoryBufferRef,
}

impl MemoryBuffer {
    pub fn new(buf: LLVMMemoryBufferRef) -> Self {
        Self { llvm_buf: buf }
    }

    /// Turns an LLVM buffer into a Vec<u8>
    #[inline]
    pub fn to_vec(&self) -> Vec<u8> {
        unsafe {
            let start = LLVMGetBufferStart(self.llvm_buf) as *const u8;
            let size = LLVMGetBufferSize(self.llvm_buf) as usize;
            slice::from_raw_parts(start, size).to_vec()
        }
    }
}

impl Drop for MemoryBuffer {
    fn drop(&mut self) {
        unsafe { LLVMDisposeMemoryBuffer(self.llvm_buf) };
    }
}
