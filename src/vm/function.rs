use vm::LLVMRef;
use vm::module::Module;
use vm::builder::BasicBlock;

use vm::libc::c_char;
use vm::llvm::analysis::{
    LLVMVerifierFailureAction,
    LLVMVerifyFunction,
};

use vm::llvm::core::{
    LLVMAddFunction,
    LLVMAppendBasicBlockInContext,
    LLVMCountParams,
    LLVMFunctionType,
    LLVMGetTypeContext,
    LLVMTypeOf,
};

use vm::llvm::prelude::{
    LLVMBool,
    LLVMTypeRef,
    LLVMValueRef,
};

use std::ffi::CString;

pub struct Function {
    func: LLVMValueRef,
}

impl Function {
    pub fn new(module: &Module, name: &str, args: &mut [LLVMTypeRef], ret: LLVMTypeRef) -> Function {
        let name = CString::new(name).unwrap();
        let func_type = unsafe {
            LLVMFunctionType(
                ret,
                args.as_mut_ptr(),
                args.len() as u32,
                0 as LLVMBool,
            )
        };

        Function {
            func: unsafe {
                LLVMAddFunction(
                    module.to_ref(),
                    name.as_ptr() as *const c_char,
                    func_type,
                )
            }
        }
    }

    pub fn new_basic_block(&self, name: &str) -> BasicBlock {
        let name = CString::new(name).unwrap();

        BasicBlock::from_ref(unsafe {
            LLVMAppendBasicBlockInContext(
                LLVMGetTypeContext(LLVMTypeOf(
                    self.to_ref(),
                )),
                self.to_ref(),
                name.as_ptr() as *const c_char,
            )
        })
    }

    pub fn arity(&self) -> u32 {
        unsafe {
            LLVMCountParams(self.to_ref()) as u32
        }
    }

    pub fn verify(&self) -> Result<(), String> {
        let status = unsafe {
            LLVMVerifyFunction(
                self.to_ref(),
                LLVMVerifierFailureAction::LLVMReturnStatusAction,
            )
        };

        match status {
            1 => Err("Unknown error".to_string()),
            _ => Ok(())
        }
    }
}

impl LLVMRef<LLVMValueRef> for Function {
    fn to_ref(&self) -> LLVMValueRef {
        self.func
    }
}