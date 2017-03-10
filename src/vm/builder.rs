use vm::LLVMRef;
use vm::context::Context;
use vm::value::Value;

use vm::libc::c_char;
use vm::llvm::core::{
    LLVMBuildAdd,
    LLVMBuildRet,
    LLVMBuildRetVoid,
    LLVMCreateBuilderInContext,
    LLVMDisposeBuilder,
    LLVMPositionBuilderAtEnd,
};

use vm::llvm::prelude::{
    LLVMBasicBlockRef,
    LLVMBuilderRef,
};

use std::ffi::CString;

pub struct Builder {
    builder: LLVMBuilderRef,
    owned:   bool,
}

impl Builder {
    pub fn new(context: &Context) -> Builder {
        Builder {
            builder: unsafe {
                LLVMCreateBuilderInContext(context.to_ref())
            },
            owned:   true,
        }
    }

    pub fn move_to_end(&mut self, basic_block: BasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.to_ref(), basic_block.to_ref());
        }
    }

    pub fn return_void(&mut self) -> Value {
        Value::from_ref(unsafe {
            LLVMBuildRetVoid(self.to_ref())
        })
    }

    pub fn return_value(&mut self, v: Value) -> Value {
        Value::from_ref(unsafe {
            LLVMBuildRet(self.to_ref(), v.to_ref())
        })
    }

    pub fn add(&mut self, lhs: Value, rhs: Value, name: &str) -> Value {
        let name = CString::new(name).unwrap();

        Value::from_ref(unsafe {
            LLVMBuildAdd(
                self.to_ref(),
                lhs.to_ref(),
                rhs.to_ref(),
                name.as_ptr() as *const c_char,
            )
        })
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                LLVMDisposeBuilder(self.builder);
            }
        }
    }
}

impl LLVMRef<LLVMBuilderRef> for Builder {
    fn to_ref(&self) -> LLVMBuilderRef {
        self.builder
    }
}

#[derive(Debug)]
pub struct BasicBlock {
    block: LLVMBasicBlockRef,
}

impl BasicBlock {
    pub fn from_ref(block_ref: LLVMBasicBlockRef) -> BasicBlock {
        BasicBlock {
            block: block_ref,
        }
    }
}

impl LLVMRef<LLVMBasicBlockRef> for BasicBlock {
    fn to_ref(&self) -> LLVMBasicBlockRef {
        self.block
    }
}