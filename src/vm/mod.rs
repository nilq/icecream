extern crate libc;
extern crate llvm_sys as llvm;

pub mod context;

pub trait LLVMRef<R> {
    fn to_ref(&self) -> R;
}