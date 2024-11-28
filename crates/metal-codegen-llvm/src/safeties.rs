// SPDX-License-Identifier: MIT
//! Safe Rust abstractions for several LLVM types

use std::{
    ffi::{c_char, CStr},
    slice,
};

use llvm_sys::{
    core::{LLVMDisposeMemoryBuffer, LLVMDisposeMessage, LLVMGetBufferSize, LLVMGetBufferStart},
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

pub struct LLVMErrorMessage {
    ptr: *mut c_char,
}

impl Default for LLVMErrorMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl LLVMErrorMessage {
    pub fn new() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }

    /// Returns a representation which can be used in LLVM functions.
    pub fn llvm(&self) -> *mut c_char {
        self.ptr
    }

    pub fn message(&self) -> String {
        unsafe { CStr::from_ptr(self.ptr).to_string_lossy().to_string() }
    }
}

impl Drop for LLVMErrorMessage {
    fn drop(&mut self) {
        unsafe { LLVMDisposeMessage(self.ptr) };
    }
}
