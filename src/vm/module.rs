use super::LLVMRef;
use super::context::Context;

use vm::libc::c_char;
use vm::llvm::analysis::{
    LLVMVerifierFailureAction,
    LLVMVerifyModule,
};

use vm::llvm::core::{
    LLVMDisposeMessage,
    LLVMDisposeModule,
    LLVMModuleCreateWithNameInContext,
    LLVMPrintModuleToString,
};

use vm::llvm::prelude::LLVMModuleRef;

use std::ffi::{CStr, CString};
use std::fmt;

pub struct Module {
    module: LLVMModuleRef,
    owned:  bool,
}

impl Module {
    pub fn new(id: &str, context: &Context) -> Module {
        let id = CString::new(id).unwrap();

        Module {
            module: unsafe {
                LLVMModuleCreateWithNameInContext(
                    id.as_ptr() as *const c_char,
                    context.to_ref(),
                )
            },
            owned: true,
        }
    }

    pub unsafe fn unown(&mut self) {
        self.owned = false;
    }

    pub fn verify(&self) -> Result<(), String> {
        let mut verify_err = 0 as *mut c_char;
        let status = unsafe {
            LLVMVerifyModule(
                self.to_ref(),
                LLVMVerifierFailureAction::LLVMReturnStatusAction,
                &mut verify_err,
            )
        };

        match status {
            1 => {
                let error;

                unsafe {
                    let err_buffer = CStr::from_ptr(verify_err);
                    error = String::from_utf8_lossy(err_buffer.to_bytes()).into_owned();;

                    LLVMDisposeMessage(verify_err)
                }

                Err(error)
            }

            _ => Ok(()),
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                LLVMDisposeModule(self.module);
            }
        }
    }
}

impl LLVMRef<LLVMModuleRef> for Module {
    fn to_ref(&self) -> LLVMModuleRef {
        self.module
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", unsafe {
            let ir_string = LLVMPrintModuleToString(self.to_ref());
            let ir        = CStr::from_ptr(ir_string).to_string_lossy()
                                            .to_owned();
            
            LLVMDisposeMessage(ir_string);
            ir
        })
    }
}