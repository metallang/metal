use std::ffi::{c_uint, CString};

use llvm_sys::{
    core::{LLVMBuildCall2, LLVMConstInt, LLVMConstStringInContext, LLVMGetNamedFunction},
    prelude::LLVMValueRef,
};
use metal_mir::{
    expr::{literals::Literal, Expr},
    parcel::Module,
    types::primitives::Primitive,
};

use crate::{CodeGenType, CodeGenValue};

fn args_to_values(llvm: &crate::LLVMRefs, module: &Module, args: &Vec<Expr>) -> Vec<LLVMValueRef> {
    let mut v = Vec::with_capacity(args.len());

    for arg in args {
        v.push(arg.codegen_value(llvm, module));
    }

    v
}

impl CodeGenValue for Expr {
    fn codegen_value(
        &self,
        llvm: &crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> llvm_sys::prelude::LLVMValueRef {
        match self {
            Self::FunctionCall(fcall) => unsafe {
                let c_fun_name = CString::new(fcall.name.clone()).unwrap();

                // TODO: handle possible errors
                let func = LLVMGetNamedFunction(llvm.module, c_fun_name.as_ptr());
                assert!(!func.is_null());

                let args_num = fcall.arguments.len();
                LLVMBuildCall2(
                    llvm.builder,
                    fcall.signature.return_type.codegen_type(llvm, module),
                    func,
                    args_to_values(llvm, module, &fcall.arguments).as_mut_ptr(),
                    args_num as c_uint,
                    c_fun_name.as_ptr(),
                )
            },
            Self::Literal(lit) => match *lit.clone() {
                Literal::Boolean(b) => unsafe {
                    if b.value {
                        LLVMConstInt(Primitive::I8.codegen_type(llvm, module), 1, 0)
                    } else {
                        LLVMConstInt(Primitive::I8.codegen_type(llvm, module), 0, 0)
                    }
                },
                Literal::Number(n) => unsafe {
                    let (sign_extend, value) = if n.value.is_negative() {
                        (1, -n.value as u64)
                    } else {
                        (0, n.value as u64)
                    };

                    LLVMConstInt(n.primitive.codegen_type(llvm, module), value, sign_extend)
                },
                Literal::String(s) => unsafe {
                    let s_len = s.value.len();
                    let c_val = CString::new(s.value).unwrap();
                    LLVMConstStringInContext(llvm.ctx, c_val.as_ptr(), s_len as c_uint, 1)
                },
            },
        }
    }
}
