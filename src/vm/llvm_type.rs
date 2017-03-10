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

bind_llvm_type!(LLVMInt1TypeInContext => int1_type);
bind_llvm_type!(LLVMInt8TypeInContext => int8_type);
bind_llvm_type!(LLVMInt16TypeInContext => int16_type);
bind_llvm_type!(LLVMInt32TypeInContext => int32_type);
bind_llvm_type!(LLVMInt64TypeInContext => int64_type);

bind_llvm_type!(LLVMDoubleTypeInContext => double_type);
bind_llvm_type!(LLVMFloatTypeInContext => float_type);

bind_llvm_type!(LLVMFP128TypeInContext => fp128_type);

bind_llvm_type!(LLVMVoidTypeInContext => void_type);
bind_llvm_type!(LLVMX86FP80TypeInContext => x86fp80_type);
bind_llvm_type!(LLVMX86MMXTypeInContext => x86mmx_type);

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

macro_rules! to_int {
    ($type_name:ty as $($alias:ty)as+ => $LLVM_type:ident($($LLVM_type_argument:expr),*)) => (
        impl VMRepresentation for $type_name {
            fn to_representation(self, context: &Context) -> Value {
                use vm::llvm::core::$LLVM_type;

                Value::from_ref(unsafe {
                    LLVMConstInt(
                        $LLVM_type(context.to_ref(), $($LLVM_type_argument), *),
                        self as $($alias)as+,
                        0 as LLVMBool,
                    )
                })
            }
        }
    );

    ($type_name:ty as $($alias:ty)as+ => $LLVM_type:ident) => (
        to_int!{
            $type_name as $($alias)as+ => $LLVM_type()
        }
    )
}

macro_rules! to_float {
    ($type_name:ty as $alias:ty => $LLVM_type:ident) => (
        impl VMRepresentation for $type_name {
            fn to_representation(self, context: &Context) -> Value {
                use vm::llvm::core::$LLVM_type;

                Value::from_ref(unsafe {
                    LLVMConstReal(
                        $LLVM_type(context.to_ref()),
                        self as $alias,
                    )
                })
            }
        }
    )
}

to_int!(bool as c_ulonglong => LLVMInt1TypeInContext);

to_int!(u8 as c_ulonglong => LLVMInt8TypeInContext);
to_int!(i8 as c_ulonglong => LLVMInt8TypeInContext);

to_int!(u16 as c_ulonglong => LLVMInt16TypeInContext);
to_int!(i16 as c_ulonglong => LLVMInt16TypeInContext);

to_int!(u32 as c_ulonglong => LLVMInt32TypeInContext);
to_int!(i32 as c_ulonglong => LLVMInt32TypeInContext);

to_int!(u64 as c_ulonglong => LLVMInt64TypeInContext);
to_int!(i64 as c_ulonglong => LLVMInt64TypeInContext);

to_int!(usize as c_ulonglong => LLVMIntTypeInContext(mem::size_of::<isize>() as c_uint * 8));
to_int!(isize as c_ulonglong => LLVMIntTypeInContext(mem::size_of::<isize>() as c_uint * 8));

to_int!(char as u32 as c_ulonglong => LLVMInt32TypeInContext)   ;

to_float!(f32 as f64 => LLVMFloatTypeInContext);
to_float!(f64 as f64 => LLVMDoubleTypeInContext);

impl<'a> VMRepresentation for &'a str {
    fn to_representation(self, context: &Context) -> Value {
        self.as_bytes().to_representation(context)
    }
}

impl<'a> VMRepresentation for &'a [u8] {
    fn to_representation(self, context: &Context) -> Value {
        Value::from_ref(unsafe {
            LLVMConstStringInContext(
                context.to_ref(),
                self.as_ptr() as *const c_char,
                self.len() as c_uint,
                1 as LLVMBool,
            )
        })
    }
}