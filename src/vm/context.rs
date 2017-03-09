use vm::llvm;

use super::LLVMRef;

use self::llvm::core::{
    LLVMContextCreate,
    LLVMContextDispose
};

use self::llvm::prelude::LLVMContextRef;

#[derive(Debug)]
pub struct Context {
    context: LLVMContextRef,
    owned:   bool,
}

impl Context {
    pub fn new() -> Context {
        Context {
            context: unsafe {
                LLVMContextCreate()
            },
            owned: true,
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                LLVMContextDispose(self.context);
            }
        }
    }
}

impl LLVMRef<LLVMContextRef> for Context {
    fn to_ref(&self) -> LLVMContextRef {
        self.context
    }
}