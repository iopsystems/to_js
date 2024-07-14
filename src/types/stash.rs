use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{Info, TypeInfo};
use crate::Wasm;
use std::sync::RwLock;

// todo: update this comment for the fact that we now store a vec
// Global stash to keep values alive across FFI boundary until the next FFI call,
// storing them in a type-erased box to be dropped when the next value is put in
// static STASH: RwLock<Option<Vec<&Box<dyn Send + Sync + 'static>>>> = RwLock::new(None);

static STASH: RwLock<Vec<Box<dyn Send + Sync + 'static>>> = RwLock::new(Vec::new());

pub fn clear_stash() {
    let mut vec = STASH.write().unwrap();
    vec.clear();
}

pub struct Stash<T>(pub T);

impl<T> From<T> for Stash<T> {
    fn from(x: T) -> Self {
        Stash(x)
    }
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
