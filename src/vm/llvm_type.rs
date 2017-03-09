use vm;

use vm::LLVMRef;
use vm::context::Context;
use vm::value::Value;

use vm::libc::{
    c_char, c_uint, c_ulonglong,
};

use vm::llvm::core::{
    LLVMConstInt,
    LLVMConstReal,
    LLVMConstStringInContext,
};

use vm::llvm::prelude::{
    LLVMBool,
    LLVMTypeRef,
};

use std::mem;

macro_rules! bind_llvm_type {
    ($LLVM_name:ident => $name:ident) => (
        pub fn $name(context: &Context) -> LLVMTypeRef {
            use vm::llvm::core::$LLVM_name;

            unsafe {
                $LLVM_name(context.to_ref())
            }
        }
    )
}

pub fn int_type(size: u32, context: &Context) -> LLVMTypeRef {
    use vm::llvm::core::LLVMIntTypeInContext;

    unsafe {
        LLVMIntTypeInContext(context.to_ref(), size as c_uint)
    }
}

pub fn array_type(element_type: LLVMTypeRef, size: u32) -> LLVMTypeRef {
    use vm::llvm::core::LLVMArrayType;

    unsafe {
        LLVMArrayType(element_type, size as c_uint)
    }
}

pub fn pointer_type(element_type: LLVMTypeRef, address_space: u32) -> LLVMTypeRef {
    use vm::llvm::core::LLVMPointerType;

    unsafe {
        LLVMPointerType(element_type, address_space as c_uint)
    }
}

pub fn vector_type(element_type: LLVMTypeRef, size: u32) -> LLVMTypeRef {
    use vm::llvm::core::LLVMVectorType;

    unsafe {
        LLVMVectorType(element_type, size as c_uint)
    }
}

pub trait VMRepresentation {
    fn to_representation(self, context: &Context) -> Value;
}

