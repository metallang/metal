use std::ffi::{c_uint, CString};

use llvm_sys::{
    core::{
        LLVMBuildAdd, LLVMBuildCall2, LLVMBuildFAdd, LLVMBuildFCmp, LLVMBuildFDiv, LLVMBuildFMul,
        LLVMBuildFRem, LLVMBuildFSub, LLVMBuildLoad2, LLVMBuildMul, LLVMBuildSDiv, LLVMBuildSRem,
        LLVMBuildStore, LLVMBuildSub, LLVMBuildUDiv, LLVMBuildURem, LLVMConstInt,
        LLVMConstStringInContext2, LLVMGetNamedFunction,
    },
    prelude::LLVMValueRef,
    LLVMRealPredicate,
};
use metal_mir::{
    expr::{literals::Literal, Expr},
    parcel::Module,
    types::primitives::Primitive,
};

use crate::{CodeGenType, CodeGenValue};

fn args_to_values(llvm: &mut crate::LLVMRefs, module: &Module, args: &[Expr]) -> Vec<LLVMValueRef> {
    let mut v = Vec::with_capacity(args.len());

    for arg in args {
        v.push(arg.codegen_value(llvm, module));
    }

    v
}

impl CodeGenValue for Expr {
    fn codegen_value(
        &self,
        llvm: &mut crate::LLVMRefs,
        module: &metal_mir::parcel::Module,
    ) -> llvm_sys::prelude::LLVMValueRef {
        match self {
            Self::FunctionCall(fcall) => unsafe {
                let c_fun_name = CString::new(fcall.signature.name.as_str()).unwrap();

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
            Self::Literal(lit) => match lit.as_ref() {
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
                    let c_val = CString::new(s.value.as_str()).unwrap();
                    LLVMConstStringInContext2(llvm.ctx, c_val.as_ptr(), s_len, 1)
                },
            },
            Self::Assignment(a) => unsafe {
                LLVMBuildStore(
                    llvm.builder,
                    a.expr.as_ref().unwrap().codegen_value(llvm, module),
                    *llvm.locals.get(a.name).unwrap(),
                )
            },
            Self::Load(l) => unsafe {
                let c_name = CString::new(l.name).unwrap();

                LLVMBuildLoad2(
                    llvm.builder,
                    l.ty.codegen_type(llvm, module),
                    *llvm.locals.get(l.name).unwrap(),
                    c_name.as_ptr(),
                )
            },
            // math
            Self::Add(m) => unsafe {
                let name = CString::new(m.result_var_name.unwrap_or("")).unwrap();
                if m.float {
                    return LLVMBuildFAdd(
                        llvm.builder,
                        m.a.codegen_value(llvm, module),
                        m.b.codegen_value(llvm, module),
                        name.as_ptr(),
                    );
                }
                LLVMBuildAdd(
                    llvm.builder,
                    m.a.codegen_value(llvm, module),
                    m.b.codegen_value(llvm, module),
                    name.as_ptr(),
                )
            },
            Self::Sub(m) => unsafe {
                let name = CString::new(m.result_var_name.unwrap_or("")).unwrap();
                if m.float {
                    return LLVMBuildFSub(
                        llvm.builder,
                        m.a.codegen_value(llvm, module),
                        m.b.codegen_value(llvm, module),
                        name.as_ptr(),
                    );
                }
                LLVMBuildSub(
                    llvm.builder,
                    m.a.codegen_value(llvm, module),
                    m.b.codegen_value(llvm, module),
                    name.as_ptr(),
                )
            },
            Self::Div(m) => unsafe {
                let name = CString::new(m.result_var_name.unwrap_or("")).unwrap();
                if m.float {
                    return LLVMBuildFDiv(
                        llvm.builder,
                        m.a.codegen_value(llvm, module),
                        m.b.codegen_value(llvm, module),
                        name.as_ptr(),
                    );
                }
                if m.signed {
                    return LLVMBuildSDiv(
                        llvm.builder,
                        m.a.codegen_value(llvm, module),
                        m.b.codegen_value(llvm, module),
                        name.as_ptr(),
                    );
                }
                LLVMBuildUDiv(
                    llvm.builder,
                    m.a.codegen_value(llvm, module),
                    m.b.codegen_value(llvm, module),
                    name.as_ptr(),
                )
            },
            Self::Mul(m) => unsafe {
                let name = CString::new(m.result_var_name.unwrap_or("")).unwrap();
                if m.float {
                    return LLVMBuildFMul(
                        llvm.builder,
                        m.a.codegen_value(llvm, module),
                        m.b.codegen_value(llvm, module),
                        name.as_ptr(),
                    );
                }
                LLVMBuildMul(
                    llvm.builder,
                    m.a.codegen_value(llvm, module),
                    m.b.codegen_value(llvm, module),
                    name.as_ptr(),
                )
            },
            Self::Percent(m) => unsafe {
                let name = CString::new(m.result_var_name.unwrap_or("")).unwrap();
                if m.float {
                    LLVMBuildFRem(
                        llvm.builder,
                        m.a.codegen_value(llvm, module),
                        m.b.codegen_value(llvm, module),
                        name.as_ptr(),
                    );
                }
                if m.signed {
                    return LLVMBuildSRem(
                        llvm.builder,
                        m.a.codegen_value(llvm, module),
                        m.b.codegen_value(llvm, module),
                        name.as_ptr(),
                    );
                }
                LLVMBuildURem(
                    llvm.builder,
                    m.a.codegen_value(llvm, module),
                    m.b.codegen_value(llvm, module),
                    name.as_ptr(),
                )
            },
            Self::Gt(m) => unsafe {
                let name = CString::new(m.result_var_name.unwrap_or("")).unwrap();
                LLVMBuildFCmp(
                    llvm.builder,
                    LLVMRealPredicate::LLVMRealOGT,
                    m.a.codegen_value(llvm, module),
                    m.b.codegen_value(llvm, module),
                    name.as_ptr(),
                )
            },
            Self::Lt(m) => unsafe {
                let name = CString::new(m.result_var_name.unwrap_or("")).unwrap();
                LLVMBuildFCmp(
                    llvm.builder,
                    LLVMRealPredicate::LLVMRealOLT,
                    m.a.codegen_value(llvm, module),
                    m.b.codegen_value(llvm, module),
                    name.as_ptr(),
                )
            },
        }
    }
}
