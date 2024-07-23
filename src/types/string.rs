use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Transform};
use crate::ToWasm;
use crate::Wasm;
use std::ffi::{CStr, CString};

// From<...> for Wasm impl
//

impl ToWasm for String {
    fn to_wasm(&self) -> Wasm {
        self.as_bytes().to_wasm()
    }
}

impl ToWasm for &str {
    fn to_wasm(&self) -> Wasm {
        self.as_bytes().to_wasm()
    }
}

impl ToWasm for CString {
    fn to_wasm(&self) -> Wasm {
        self.as_bytes().to_wasm()
    }
}

impl ToWasm for &CStr {
    fn to_wasm(&self) -> Wasm {
        self.to_bytes().to_wasm()
    }
}

// HasNiche impl
//

impl HasNiche for String {
    const N: Niche = Niche::LowBitsOne;
}

impl HasNiche for &str {
    const N: Niche = Niche::LowBitsOne;
}

impl HasNiche for CString {
    const N: Niche = Niche::HighBitsNaN;
}

impl HasNiche for &CStr {
    const N: Niche = Niche::HighBitsNaN;
}

// TypeInfo impl
//

impl_typeinfo! {
    [String,  ArrayType::U8, true, Transform::String],
    [&str,    ArrayType::U8, true, Transform::String],
    [CString, ArrayType::U8, true, Transform::String],
    [&CStr,   ArrayType::U8, true, Transform::String],
}
