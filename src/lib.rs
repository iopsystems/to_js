#[cfg(feature = "proc-macro")]
extern crate js_proc_macro;

#[cfg(feature = "proc-macro")]
pub use js_proc_macro::*;

#[macro_use]
mod typeinfo;
mod niche;
mod types;

pub use typeinfo::TypeInfo;
pub use types::packed::*;
pub use types::stash::Stash;

// Core struct that represents values that can be returned across the FFI boundary
pub struct Wasm(f64);

impl Wasm {
    pub fn value(self) -> f64 {
        self.0
    }
}

/// This macro is part of the API surface of this package. The other part is the #[js] proc macro, which calls this one.
/// You can wrap a series of function definitions in this macro in order to export them to JavaScript via WebAssembly.
/// Note: Unlike the #[js] proc macro, to_js! requires that all functions have an explicit return type, even if it is ().
#[macro_export]
macro_rules! to_js {
    ($( $(#[$meta:meta])* $vis:vis fn $name:ident($($arg:ident : $typ:ty),*) -> $ret:ty $body:block )*) => {
        $(
            // Define the original function
            $(#[$meta])*
            $vis fn $name($($arg: $typ),*) -> $ret $body

            // Define exported functions, using a const block in order to allow repetition of the Rust-side
            // function names (call and info) if multiple functions are exported in the same outer scope.
            const _: () = {
                use $crate::{Wasm, TypeInfo, U8Octet};

                // Define the exported function, which returns an f64-encoded Wasm value
                #[export_name = concat!(stringify!($name))]
                pub extern "C" fn call($($arg: $typ),*) -> f64 {
                    let value = $name($($arg),*);
                    Wasm::from(value).value()
                }

                // Define a companion function which returns the info needed to interpret the encoding.
                #[export_name = concat!(stringify!($name), "_info_")]
                pub extern "C" fn type_info() -> f64 {
                    let info = <$ret as TypeInfo>::type_info();
                    let mut octet = info.to_octet();
                    Wasm::from(U8Octet(octet)).value()
                }
            };
        )*
    };
}

// Convenience functions for JS-side resource lifetime management
//

pub fn allocate<T>(x: T) -> *mut T {
    Box::into_raw(Box::new(x))
}

pub unsafe fn as_ref<'a, T>(ptr: *const T) -> &'a T {
    unsafe { &*ptr }
}

pub unsafe fn as_mut_ref<'a, T>(ptr: *mut T) -> &'a mut T {
    unsafe { &mut *ptr }
}

pub unsafe fn deallocate<T>(ptr: *mut T) {
    let x = unsafe { Box::from_raw(ptr) };
    drop(x)
}

/// A trick: We embed most of the JavaScript required to use the compiled .wasm file inside inside the .wasm file.
/// We export this constant directly into the resulting .wasm binary, where its value points to the slice descriptor.
/// The first element of the descriptor is a pointer to the string contents, and the second element is its length.
#[used]
#[export_name = "JS"]
pub static JS: &[u8] = &include_bytes!("./lib.js").as_slice();
