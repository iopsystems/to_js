use crate::types::stash::Stash;
use crate::Wasm;
use std::ffi::{CStr, CString};

// Error strings are a special internal type used to limit the Result Err variant
// to be a C-style null-terminated string so that it can be encoded in 32 bits.

// Stash safety:
// We rely on the Stash for CStrings to ensure that value lives long enough for data to be copied out.
// This does not conflict with any user uses of Stash because we only evaluate this function if we are
// in the error variant of a Result return value.

pub(crate) trait ErrorString {
    fn to_u32(self) -> u32;
}

impl ErrorString for () {
    fn to_u32(self) -> u32 {
        c"".to_u32()
    }
}

impl ErrorString for String {
    fn to_u32(self) -> u32 {
        match CString::new(self) {
            Ok(value) => {
                // The Wasm encoding of a CString is a (ptr, len) pair
                let wasm = Wasm::from(Stash(value)).value();
                // Extract and return the ptr, which is stored in the low bits
                wasm.to_bits() as u32
            }
            Err(_) => c"Meta-error: The original error string contained a NUL byte.".to_u32(),
        }
    }
}

impl ErrorString for &str {
    fn to_u32(self) -> u32 {
        String::from(self).to_u32()
    }
}

impl ErrorString for CString {
    fn to_u32(self) -> u32 {
        self.as_c_str().to_u32()
    }
}

impl ErrorString for &CStr {
    fn to_u32(self) -> u32 {
        self.as_ptr() as u32
    }
}
