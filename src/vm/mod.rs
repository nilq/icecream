extern crate libc;
extern crate llvm_sys as llvm;

pub mod context;
pub mod module;
pub mod value;
pub mod llvm_type;
pub mod llvm_builder;

pub trait LLVMRef<R> {
    fn to_ref(&self) -> R;
}