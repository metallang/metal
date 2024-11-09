//! LLVM Codegen Library

use llvm_sys::{
    core::{
        LLVMAddFunction, LLVMAppendBasicBlockInContext, LLVMArrayType2, LLVMDoubleTypeInContext,
        LLVMFP128TypeInContext, LLVMFloatTypeInContext, LLVMFunctionType, LLVMIntTypeInContext,
        LLVMPositionBuilderAtEnd, LLVMSetLinkage, LLVMStructTypeInContext, LLVMVoidTypeInContext,
    },
    prelude::*,
    LLVMLinkage,
};
use metal_mir::{stmt::functiondef::FunctionDefinition, types::Type};

pub struct CodeGen {
    module_name: &'static str,
    ctx: LLVMContextRef,
    builder: LLVMBuilderRef,
    module: LLVMModuleRef,
}

impl CodeGen {
    pub fn get_types(&self, types: Vec<Type>) -> Vec<LLVMTypeRef> {
        let mut v = Vec::with_capacity(types.len());
        for t in types {
            v.push(self.get_type(t))
        }
        v
    }

    pub fn get_type(&self, t: metal_mir::types::Type) -> LLVMTypeRef {
        match t {
            metal_mir::types::Type::Primitive(p) => unsafe {
                self.get_llvm_type_primitive(*(p.clone()))
            },
            metal_mir::types::Type::Composite(c) => unsafe {
                match *c {
                    metal_mir::types::Composite::Tuple(t) => LLVMStructTypeInContext(
                        self.ctx,
                        self.get_types(t.types.clone()).as_mut_ptr(),
                        t.types.len().try_into().unwrap(),
                        0,
                    ),
                    metal_mir::types::Composite::Array(a) => {
                        LLVMArrayType2(self.get_type(a.item_type), a.size)
                    }
                }
            },
            metal_mir::types::Type::Function(f) => unsafe {
                LLVMFunctionType(
                    self.get_type(f.return_type),
                    self.get_types(f.arguments.clone()).as_mut_ptr(),
                    f.arguments.len().try_into().unwrap(),
                    0,
                )
            },
        }
    }

    /// # Safety
    /// This just calls a bunch of LLVM functions to return
    /// a function.
    pub unsafe fn get_llvm_type_primitive(
        &self,
        p: metal_mir::types::primitives::Primitive,
    ) -> LLVMTypeRef {
        match p {
            // Integers (signed)
            metal_mir::types::primitives::Primitive::I8 => LLVMIntTypeInContext(self.ctx, 8),
            metal_mir::types::primitives::Primitive::I16 => LLVMIntTypeInContext(self.ctx, 16),
            metal_mir::types::primitives::Primitive::I32 => LLVMIntTypeInContext(self.ctx, 32),
            metal_mir::types::primitives::Primitive::I64 => LLVMIntTypeInContext(self.ctx, 64),
            metal_mir::types::primitives::Primitive::I128 => LLVMIntTypeInContext(self.ctx, 128),

            // Integers (unsigned)
            metal_mir::types::primitives::Primitive::U8 => LLVMIntTypeInContext(self.ctx, 8),
            metal_mir::types::primitives::Primitive::U16 => LLVMIntTypeInContext(self.ctx, 16),
            metal_mir::types::primitives::Primitive::U32 => LLVMIntTypeInContext(self.ctx, 32),
            metal_mir::types::primitives::Primitive::U64 => LLVMIntTypeInContext(self.ctx, 64),
            metal_mir::types::primitives::Primitive::U128 => LLVMIntTypeInContext(self.ctx, 128),

            // Floats; TODO: support f16, maybe f80 too?
            metal_mir::types::primitives::Primitive::F32 => LLVMFloatTypeInContext(self.ctx),
            metal_mir::types::primitives::Primitive::F64 => LLVMDoubleTypeInContext(self.ctx),
            metal_mir::types::primitives::Primitive::F128 => LLVMFP128TypeInContext(self.ctx),

            // Strings
            metal_mir::types::primitives::Primitive::Literal(length) => {
                let char_ty = LLVMIntTypeInContext(self.ctx, 8);
                LLVMArrayType2(char_ty, length)
            }

            // NOTE: Void is represented by an empty tuple. `()`
            // Void
            metal_mir::types::primitives::Primitive::Void => LLVMVoidTypeInContext(self.ctx),
        }
    }

    pub fn statement(&self, stmt: metal_mir::stmt::Statement) {
        match stmt {
            metal_mir::stmt::Statement::FunctionDefine(def) => {
                self.function_definition(*def);
            }
        }
    }

    pub fn function_definition(&self, definition: FunctionDefinition) -> LLVMValueRef {
        let fun_name: String = if definition.module.name != "main" {
            // mangles the function name into the following format:
            // module (split by submodule name length) +
            // function name (with its length also preceding the name)

            let mut name = "".to_string();
            for module in self.module_name.split('.') {
                let m1 = module.len().to_string() + module;
                let m = m1.as_str();
                name += m;
            }
            let n = name
                + definition.signature.name.len().to_string().as_str()
                + definition.signature.name.as_str();
            n
        } else {
            definition.signature.name.clone()
        };

        let linkage = match definition.signature.vis {
            metal_mir::types::visibility::Visibility::Parcel => LLVMLinkage::LLVMInternalLinkage,
            metal_mir::types::visibility::Visibility::Private => LLVMLinkage::LLVMInternalLinkage,
            metal_mir::types::visibility::Visibility::Public => LLVMLinkage::LLVMExternalLinkage,
        };

        unsafe {
            let function = LLVMAddFunction(
                self.module,
                fun_name.as_ptr() as *const i8,
                self.get_type(metal_mir::types::Type::Function(Box::new(
                    definition.signature.clone(),
                ))),
            );
            LLVMSetLinkage(function, linkage);

            let entry_block = LLVMAppendBasicBlockInContext(self.ctx, function, c"entry".as_ptr());
            LLVMPositionBuilderAtEnd(self.builder, entry_block);

            for stmt in definition.body {
                self.statement(stmt);
            }

            function
        }
    }
}
