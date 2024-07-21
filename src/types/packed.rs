use crate::typeinfo::{ArrayType, Transform};
use crate::Wasm;

// Packed arrays (encoded into 64 bits).
// These are returned as arrays but make use of special
// preallocated arrays on the JavaScript side that
// reinterpret the memory of a single-element Float64Array.

#[derive(Copy, Clone)]
pub struct U8Octet(pub [u8; 8]);

#[derive(Copy, Clone)]
pub struct I8Octet(pub [i8; 8]);

#[derive(Copy, Clone)]
pub struct U16Quartet(pub [u16; 4]);

#[derive(Copy, Clone)]
pub struct I16Quartet(pub [i16; 4]);

#[derive(Copy, Clone)]
pub struct U32Pair(pub [u32; 2]);

#[derive(Copy, Clone)]
pub struct I32Pair(pub [i32; 2]);

#[derive(Copy, Clone)]
pub struct F32Pair(pub [f32; 2]);

// From<...> for Wasm impl
//

impl From<U8Octet> for Wasm {
    fn from(x: U8Octet) -> Self {
        let [a, b, c, d, e, f, g, h] = x.0;
        f64::from_bits(
            ((h as u64) << 56)
                | ((g as u64) << 48)
                | ((f as u64) << 40)
                | ((e as u64) << 32)
                | ((d as u64) << 24)
                | ((c as u64) << 16)
                | ((b as u64) << 8)
                | a as u64,
        )
        .into()
    }
}

impl From<I8Octet> for Wasm {
    fn from(x: I8Octet) -> Self {
        U8Octet(x.0.map(|x| x as u8)).into()
    }
}

impl From<U16Quartet> for Wasm {
    fn from(x: U16Quartet) -> Self {
        let [a, b, c, d] = x.0;
        f64::from_bits(((d as u64) << 48) | ((c as u64) << 32) | ((b as u64) << 16) | a as u64)
            .into()
    }
}

impl From<I16Quartet> for Wasm {
    fn from(x: I16Quartet) -> Self {
        U16Quartet(x.0.map(|x| x as u16)).into()
    }
}

impl From<U32Pair> for Wasm {
    fn from(x: U32Pair) -> Self {
        let [a, b] = x.0;
        f64::from_bits(((b as u64) << 32) | a as u64).into()
    }
}

impl From<I32Pair> for Wasm {
    fn from(x: I32Pair) -> Self {
        U32Pair(x.0.map(|x| x as u32)).into()
    }
}

impl From<F32Pair> for Wasm {
    fn from(x: F32Pair) -> Self {
        U32Pair(x.0.map(f32::to_bits)).into()
    }
}

// HasNiche impl
// (none since these types has no available niche; all 64 bits are meaningful)

// TypeInfo impl
//

impl_typeinfo! {
    [U8Octet,    ArrayType::None, false, Transform::U8Octet],
    [I8Octet,    ArrayType::None, false, Transform::I8Octet],
    [U16Quartet, ArrayType::None, false, Transform::U16Quartet],
    [I16Quartet, ArrayType::None, false, Transform::I16Quartet],
    [U32Pair,    ArrayType::None, false, Transform::U32Pair],
    [I32Pair,    ArrayType::None, false, Transform::I32Pair],
    [F32Pair,    ArrayType::None, false, Transform::F32Pair],
}
