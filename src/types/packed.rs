use crate::typeinfo::{ArrayType, Transform};
use crate::ToWasm;
use crate::Wasm;

// Packed arrays (encoded into 64 bits).
// These are returned as arrays but make use of special
// preallocated arrays on the JavaScript side that
// reinterpret the memory of a single-element Float64Array.

pub struct U8Octet(pub [u8; 8]);
pub struct I8Octet(pub [i8; 8]);
pub struct U16Quartet(pub [u16; 4]);
pub struct I16Quartet(pub [i16; 4]);
pub struct U32Pair(pub [u32; 2]);
pub struct I32Pair(pub [i32; 2]);
pub struct F32Pair(pub [f32; 2]);

// From<...> for Wasm impl
//

impl ToWasm for U8Octet {
    fn to_wasm(&self) -> Wasm {
        let [a, b, c, d, e, f, g, h] = self.0;
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
        .to_wasm()
    }
}

impl ToWasm for I8Octet {
    fn to_wasm(&self) -> Wasm {
        U8Octet(self.0.map(|x| x as u8)).to_wasm()
    }
}

impl ToWasm for U16Quartet {
    fn to_wasm(&self) -> Wasm {
        let [a, b, c, d] = self.0;
        f64::from_bits(((d as u64) << 48) | ((c as u64) << 32) | ((b as u64) << 16) | a as u64)
            .to_wasm()
    }
}

impl ToWasm for I16Quartet {
    fn to_wasm(&self) -> Wasm {
        U16Quartet(self.0.map(|x| x as u16)).to_wasm()
    }
}

impl ToWasm for U32Pair {
    fn to_wasm(&self) -> Wasm {
        let [a, b] = self.0;
        f64::from_bits(((b as u64) << 32) | a as u64).to_wasm()
    }
}

impl ToWasm for I32Pair {
    fn to_wasm(&self) -> Wasm {
        U32Pair(self.0.map(|x| x as u32)).to_wasm()
    }
}

impl ToWasm for F32Pair {
    fn to_wasm(&self) -> Wasm {
        U32Pair(self.0.map(f32::to_bits)).to_wasm()
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
