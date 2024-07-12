use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Transform};
use crate::Wasm;

pub(crate) trait Number: 'static + Send + Sync {
    fn to_f64(self) -> f64;
}

macro_rules! impl_number {
    ($( $type:ty $(,)? )*) => {
        $(
            impl Number for $type {
                fn to_f64(self) -> f64 {
                    self as f64
                }
            }

            impl HasNiche for $type {
                const N: Niche = Niche::HighBitsNaN;
            }
        )*
    };
}

impl_number!(i8, i16, i32, u8, u16, u32, f32, f64, usize, isize);

impl Number for u64 {
    fn to_f64(self) -> f64 {
        f64::from_bits(self)
    }
}

impl Number for i64 {
    fn to_f64(self) -> f64 {
        (self as u64).to_f64()
    }
}

// From<...> for Wasm impl
//

impl<T: Number> From<T> for Wasm {
    fn from(x: T) -> Self {
        Wasm(x.to_f64())
    }
}

// HasNiche impl
// (There's no blanket implementation for Number since since not *all* numbers
//  are niche; u64 and i64 have no available niche. We implemented it for
//  all supported primitives using the impl_number! macro.)

// TypeInfo impl
//

impl_typeinfo! {
    [u8,    ArrayType::U8,  false, Transform::Identity],
    [i8,    ArrayType::I8,  false, Transform::Identity],
    [u16,   ArrayType::U16, false, Transform::Identity],
    [i16,   ArrayType::I16, false, Transform::Identity],
    [u32,   ArrayType::U32, false, Transform::Identity],
    [u64,   ArrayType::U64, false, Transform::AsU64],
    [i32,   ArrayType::I32, false, Transform::Identity],
    [i64,   ArrayType::I64, false, Transform::AsI64],
    [f32,   ArrayType::F32, false, Transform::Identity],
    [f64,   ArrayType::F64, false, Transform::Identity],
    [usize, ArrayType::U32, false, Transform::Identity],
    [isize, ArrayType::I32, false, Transform::Identity],
}
