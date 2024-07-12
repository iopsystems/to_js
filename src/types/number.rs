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

impl_number!(i8, i16, i32, u8, u16, u32, f32, f64);

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

// Into<Wasm> impl via the From trait
//

impl<T: Number> From<T> for Wasm {
    fn from(x: T) -> Self {
        Wasm(x.to_f64())
    }
}

// Wrappable impl
// (There's no blanket implementation for Number since since not *all* numbers
//  are niche; u64 and i64 have no available niche. We implemented it for
//  all supported primitives using the impl_number! macro.)

// TypeInfo impl
//

impl_typeinfo! {
    [u8,  ArrayType::U8,  Transform::Identity, false],
    [i8,  ArrayType::I8,  Transform::Identity, false],
    [u16, ArrayType::U16, Transform::Identity, false],
    [i16, ArrayType::I16, Transform::Identity, false],
    [u32, ArrayType::U32, Transform::Identity, false],
    [u64, ArrayType::U64, Transform::AsU64,    false],
    [i32, ArrayType::I32, Transform::Identity, false],
    [i64, ArrayType::I64, Transform::AsI64,    false],
    [f32, ArrayType::F32, Transform::Identity, false],
    [f64, ArrayType::F64, Transform::Identity, false],
}
