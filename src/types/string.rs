use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Transform};
use crate::IntoWasm;
use crate::Wasm;
use std::ffi::{CStr, CString};

// From<...> for Wasm impl
//

impl IntoWasm for String {
    fn into_wasm(&self) -> Wasm {
        self.as_bytes().into()
    }
}

impl From<&str> for Wasm {
    fn from(x: &str) -> Self {
        x.as_bytes().into()
    }
}

impl From<&CStr> for Wasm {
    fn from(x: &CStr) -> Self {
        x.to_bytes().into()
    }
}

impl From<&CString> for Wasm {
    fn from(x: &CString) -> Self {
        x.as_bytes().into()
    }
}

// HasNiche impl
//

impl HasNiche for &String {
    const N: Niche = Niche::LowBitsOne;
}

impl HasNiche for &str {
    const N: Niche = Niche::LowBitsOne;
}

impl HasNiche for &CStr {
    const N: Niche = Niche::HighBitsNaN;
}

impl HasNiche for &CString {
    const N: Niche = Niche::HighBitsNaN;
}

// TypeInfo impl
//

impl_typeinfo! {
    [&String,  ArrayType::U8, true, Transform::String],
    [&str,     ArrayType::U8, true, Transform::String],
    [&CString, ArrayType::U8, true, Transform::String],
    [&CStr,    ArrayType::U8, true, Transform::String],
}
