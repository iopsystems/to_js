use crate::{types::stash::Stash, IntoWasm};
use std::ffi::{CStr, CString};

// Error strings are a special internal type used to limit the Result Err variant
// to be a C-style null-terminated string so that it can be encoded in 32 bits.

pub(crate) trait ErrorString {
    fn to_u32(&self) -> u32;
}

impl ErrorString for () {
    fn to_u32(&self) -> u32 {
        c"".to_u32()
    }
}

impl ErrorString for String {
    fn to_u32(&self) -> u32 {
        match CString::new(self.clone()) {
            Ok(value) => value.to_u32(),
            Err(_) => c"Meta-error: The original error string contained a NUL byte.".to_u32(),
        }
    }
}

impl ErrorString for &str {
    fn to_u32(&self) -> u32 {
        String::from(*self).to_u32()
    }
}

impl ErrorString for CString {
    fn to_u32(&self) -> u32 {
        // The Wasm encoding of a CString is a (ptr, len) pair
        let wasm = Stash::new(self.clone()).into_wasm().value();
        // Extract and return the ptr, which is stored in the low bits
        wasm.to_bits() as u32
    }
}

impl ErrorString for &CStr {
    fn to_u32(&self) -> u32 {
        self.as_ptr() as u32
    }
}

impl<T: ErrorString> ErrorString for &T {
    fn to_u32(&self) -> u32 {
        (*self).to_u32()
    }
}
