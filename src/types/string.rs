use crate::typeinfo::{ArrayType, Transform};
use crate::niche::{Niche, HasNiche};
use crate::Wasm;
use std::ffi::{CStr, CString};

// Into<Wasm> impl via the From trait
//

impl From<&String> for Wasm {
    fn from(x: &String) -> Self {
        x.as_bytes().into()
    }
}

impl From<&str> for Wasm {
    fn from(x: &str) -> Self {
        x.as_bytes().into()
    }
}

impl From<&CStr> for Wasm {
    fn from(x: &CStr) -> Self {
        x.as_ptr().into() // Note: This encoding is relied on by ErrorString
    }
}

impl From<&CString> for Wasm {
    fn from(x: &CString) -> Self {
        x.as_c_str().into()
    }
}

// Wrappable impl
//

impl HasNiche for &String {
    const N: Niche = Niche::LowBitsZero;
}

impl HasNiche for &str {
    const N: Niche = Niche::LowBitsZero;
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
    [&String,  ArrayType::U8,   Transform::String,  true],
    [&str,     ArrayType::U8,   Transform::String,  true],
    [&CString, ArrayType::None, Transform::CString, false],
    [&CStr,    ArrayType::None, Transform::CString, false],
}
