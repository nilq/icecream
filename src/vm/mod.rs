extern crate libc;
extern crate llvm_sys as llvm;

pub mod context;
pub mod module;
pub mod value;
pub mod llvm_type;
pub mod builder;
pub mod function;

pub trait LLVMRef<R> {
    fn to_ref(&self) -> R;
}