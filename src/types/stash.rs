use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{Info, TypeInfo};
use crate::IntoWasm;
use crate::ToWasm;
use crate::Wasm;
use std::sync::RwLock;

// Global stash to keep values alive across FFI boundary until the next FFI call,
// storing them in a vector of type-erased boxes, to be dropped when the next value is put in.
static STASH: RwLock<Vec<Box<dyn Send + Sync + 'static>>> = RwLock::new(Vec::new());

pub struct Stash<T>(pub T);

impl<T> From<T> for Stash<T> {
    fn from(x: T) -> Self {
        Stash(x)
    }
}

pub fn clear_stash() {
    STASH.write().unwrap().clear();
}

// IntoWasm impl
//

impl<T> IntoWasm for Stash<T>
where
    T: Send + Sync + 'static,
    for<'a> &'a T: ToWasm, // This bound is required to be ToWasm since we need to Wasmify a reference to T
{
    fn into_wasm(self) -> Wasm {
        let value = self.0;
        let wasm = (&value).to_wasm();
        STASH.write().unwrap().push(Box::new(value));
        wasm
    }
}

// HasNiche impl
//

impl<T: HasNiche> HasNiche for Stash<T> {
    const N: Niche = <&T>::N;
}

// TypeInfo impl
//

impl<T: TypeInfo> TypeInfo for Stash<T> {
    fn type_info() -> Info {
        <&T>::type_info()
    }
}
