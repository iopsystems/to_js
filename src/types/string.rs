use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Transform};
use crate::{ToWasm, Wasm};
use std::ffi::{CStr, CString};

// ToWasm impl
//
// We only impl ToWasm on the *reference* forms (&String, &str, &CString, &CStr).
// Without this constraint, returning an owned String / CString from a #[js] function
// would compile and produce a use-after-free: the macro's `value.into_wasm().value()`
// would drop the owned value before the f64-encoded (ptr, len) reached JS. Owned
// strings can still be returned via KeepAlive<String> or KeepAlive<CString>, which
// move the value into a static stash that lives across the FFI boundary.

impl ToWasm for &String {
    fn to_wasm(&self) -> Wasm {
        self.as_bytes().to_wasm()
    }
}

impl ToWasm for &str {
    fn to_wasm(&self) -> Wasm {
        self.as_bytes().to_wasm()
    }
}

impl ToWasm for &CString {
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
// All of these are encoded as (ptr, len) pairs with `is_array: true` in TypeInfo,
// so they must use the LowBitsOne niche to agree with the JS-side niche selector
// (which keys off `isArray`).

impl HasNiche for &String {
    const N: Niche = Niche::LowBitsOne;
}

impl HasNiche for &str {
    const N: Niche = Niche::LowBitsOne;
}

impl HasNiche for &CString {
    const N: Niche = Niche::LowBitsOne;
}

impl HasNiche for &CStr {
    const N: Niche = Niche::LowBitsOne;
}

// TypeInfo impl
//

impl_typeinfo! {
    [&String,  ArrayType::U8, true, Transform::String],
    [&str,     ArrayType::U8, true, Transform::String],
    [&CString, ArrayType::U8, true, Transform::String],
    [&CStr,    ArrayType::U8, true, Transform::String],
}
