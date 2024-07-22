#[cfg(feature = "proc-macro")]
extern crate js_proc_macro;

#[cfg(feature = "proc-macro")]
pub use js_proc_macro::*;

#[macro_use]
mod typeinfo;
mod niche;
mod types;

pub use typeinfo::TypeInfo;
pub use types::dynamic::{Dynamic, DynamicArray};
pub use types::packed::*;
pub use types::stash::{clear_stash, Stash};

// Wasm is the central type of this library and represents values that can be returned across the FFI boundary.
// Individual types that we want to be serializable implement Into<Wasm> via impls of the `From` trait.
pub struct Wasm(f64);

impl Wasm {
    pub fn value(&self) -> f64 {
        self.0
    }
}

// For types that can be efficiently dereferenced, we allow their references to be serialized.
// The workings of this library mean that functions are only converted into `Wasm`s when we are
// confident that they will outlive the Rust side of the FFI call and be available on the JS side.
// Adding this blanket implementation allows us to use basic types (bool, numbers, strings) together
// with the more advanced features this library provides (Stash and Dynamic), since stashing a value
// requires that a reference to that value is Into<Wasm>.
impl<T> From<&T> for Wasm
where
    T: Copy + Into<Wasm>,
{
    fn from(x: &T) -> Self {
        T::into(*x)
    }
}

trait ToWasm {
    fn to_wasm(&self) -> Wasm;
}

// The Wasmification process will call into_wasm.
trait IntoWasm {
    fn into_wasm(self) -> Wasm;
}

impl<T: ToWasm> IntoWasm for T {
    fn into_wasm(self) -> Wasm {
        self.to_wasm()
    }
}

impl<T> ToWasm for &T
where
    T: Copy + ToWasm,
{
    fn to_wasm(&self) -> Wasm {
        ToWasm::to_wasm(*self)
    }
}

/// This macro is part of the API surface of this package. The other part is the #[js] proc macro, which calls this one.
/// You can wrap a series of function definitions in this macro in order to export them to JavaScript via WebAssembly.
/// Note: Unlike the #[js] proc macro, to_js! requires that all functions have an explicit return type, even if it is (),
/// since we use that macro capture ($ret) to figure out the TypeInfo for each function the user wants to export.
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
                    $crate::clear_stash();
                    let value = $name($($arg),*);
                    Wasm::from(value).value()
                }

                // Define a companion function which returns the info needed to interpret the encoding.
                #[export_name = concat!(stringify!($name), "_info_")]
                pub extern "C" fn type_info() -> f64 {
                    let info = <$ret as TypeInfo>::type_info();
                    Wasm::from(info).value()
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

pub unsafe fn deallocate<T>(ptr: *mut T) {
    let x = unsafe { Box::from_raw(ptr) };
    drop(x) // strictly speaking, we could omit this, but we include it for clarity.
}

/// A trick: We embed most of the JavaScript required to use the compiled .wasm file inside of the file itself by
/// exporting this constant directly. Its value is a two-element slice descriptor whose first element is a pointer
/// to the string contents and whose second element is the string's length.
/// Note: We currently embed unminified JavaScript. With basic zip compression, the difference in .wasm.zip size
/// is a little over half a kilobyte between the unminified code and a version minified using esbuild:
/// https://esbuild.github.io/try/#dAAwLjIzLjAALS1taW5pZnkA.
#[used]
#[export_name = "JS"]
pub static JS: &[u8] = &include_bytes!("./lib.js").as_slice();
