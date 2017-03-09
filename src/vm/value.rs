use vm;

use vm::LLVMRef;

use vm::llvm::core::{
    LLVMDisposeMessage,
    LLVMPrintValueToString,    
};

use vm::llvm::prelude::LLVMValueRef;

use std::ffi::CStr;
use std::fmt;

pub struct Value {
    value: LLVMValueRef
}

impl Value {
    pub fn from_ref(value: LLVMValueRef) -> Value {
        Value {
            value: value,
        }
    }
}

impl LLVMRef<LLVMValueRef> for Value {
    fn to_ref(&self) -> LLVMValueRef {
        self.value
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", unsafe {
            let ir_string = LLVMPrintValueToString(self.to_ref());
            let ir        = CStr::from_ptr(ir_string).to_string_lossy()
                                                     .into_owned();
            
            LLVMDisposeMessage(ir_string);
            ir
        })
    }
}