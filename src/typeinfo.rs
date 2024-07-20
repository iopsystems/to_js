// TypeInfo impl so that the JavaScript code can correctly handle values of every supported type
//

use crate::{U8Octet, Wasm};

// TypedArray type if the return value is to be converted to a typed array
// Variant order is mirrored in an array on the JavaScript side.
pub enum ArrayType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    None,
}

// Transformation function to be used before returning the value to the caller
// Variant order is mirrored in an array on the JavaScript side.
pub enum Transform {
    // Store the "array" Transforms used for packed arrays
    // in the same order (with the same discriminant) as the
    // corresponding ArrayType so that the JavaScript code can
    // concisely map between the two enums via their indexes.
    U8Octet = ArrayType::U8 as isize,
    I8Octet = ArrayType::I8 as isize,
    U16Quartet = ArrayType::U16 as isize,
    I16Quartet = ArrayType::I16 as isize,
    U32Pair = ArrayType::U32 as isize,
    I32Pair = ArrayType::I32 as isize,
    F32Pair = ArrayType::F32 as isize,
    AsU64 = ArrayType::U64 as isize,
    AsI64 = ArrayType::I64 as isize,
    Identity,
    Void,
    Bool,
    String,
    Dynamic,
}

pub struct Info {
    array_type: ArrayType,
    transform: Transform,
    is_array: bool,
    is_option: bool,
    is_result: bool,
}

// Helper functions to upgrade a basic type into an array, option, and/or result.
impl Info {
    pub fn new(array_type: ArrayType, is_array: bool, transform: Transform) -> Self {
        Self {
            array_type,
            transform,
            is_array,
            is_option: false,
            is_result: false,
        }
    }

    pub(crate) fn array(self) -> Info {
        // If array_type is None, then this type cannot be placed into a (typed) array.
        // An example of this is U32Pair; we cannot ship &[U32Pair] across the FFI boundary.
        debug_assert!(!matches!(self.array_type, ArrayType::None));
        debug_assert!(!self.is_array);
        Info {
            is_array: true,
            ..self
        }
    }

    pub(crate) fn option(self) -> Info {
        debug_assert!(!self.is_option);
        Info {
            is_option: true,
            ..self
        }
    }

    pub(crate) fn result(self) -> Info {
        debug_assert!(!self.is_result);
        Info {
            is_result: true,
            ..self
        }
    }

    pub(crate) fn identity_transform(self) -> Info {
        Info {
            transform: Transform::Identity,
            ..self
        }
    }
}

impl From<Info> for Wasm {
    fn from(x: Info) -> Self {
        // Encode the type info in the bottom 32 bits
        U8Octet([
            (x.is_result as u8) | ((x.is_option as u8) << 1),
            x.is_array as u8,
            x.array_type as u8,
            x.transform as u8,
            0,
            0,
            0,
            0,
        ])
        .into()
    }
}

// Trait representing the ability to get type info for a type.
// Every type that implements Wasm should implement this trait.
pub trait TypeInfo {
    fn type_info() -> Info;
}

#[macro_export]
macro_rules! impl_typeinfo {
    ($( [$type:ty, $array_type:expr, $is_array:ident, $transform:expr] $(,)? )*) => {
        $(
            impl $crate::typeinfo::TypeInfo for $type {
                fn type_info() -> $crate::typeinfo::Info {
                    $crate::typeinfo::Info::new($array_type, $is_array, $transform)
                }
            }

            impl $crate::typeinfo::TypeInfo for &$type {
                fn type_info() -> $crate::typeinfo::Info {
                    $crate::typeinfo::Info::new($array_type, $is_array, $transform)
                }
            }
        )*
    };
}
