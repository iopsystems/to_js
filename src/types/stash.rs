use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{Info, TypeInfo};
use crate::IntoWasm;
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

// From<...> for Wasm impl
//

impl<T> From<Stash<T>> for Wasm
where
    T: Send + Sync + 'static,
    for<'a> &'a T: Into<Wasm>,
{
    fn from(x: Stash<T>) -> Self {
        let value = x.0;
        let wasm = (&value).into();
        let mut vec = STASH.write().unwrap();
        vec.push(Box::new(value));
        wasm
    }
}

impl<T> IntoWasm for Stash<T>
where
    T: Send + Sync + 'static,
    for<'a> &'a T: Into<Wasm>,
{
    fn into_wasm(self) -> Wasm {
        let value = self.0;
        let wasm = (&value).into();
        let mut vec = STASH.write().unwrap();
        vec.push(Box::new(value));
        wasm
    }
}
// HasNiche impl
//

impl<T> HasNiche for Stash<T>
where
    for<'a> &'a T: HasNiche,
    Stash<T>: Into<Wasm>,
{
    const N: Niche = <&T>::N;
}

// TypeInfo impl
//

impl<T> TypeInfo for Stash<T>
where
    for<'a> &'a T: TypeInfo,
{
    // Info for a Stash<T> is the same as the Info for &T
    fn type_info() -> Info {
        <&T>::type_info()
    }
}
