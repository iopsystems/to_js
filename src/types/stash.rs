use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{Info, TypeInfo};
use crate::IntoWasm;
use crate::ToWasm;
use crate::Wasm;
use std::marker::PhantomData;
use std::sync::RwLock;

// Global stash to keep values alive across FFI boundary until the next FFI call,
// storing them in a vector of type-erased boxes, to be dropped when the next value is put in.
static STASH: RwLock<Vec<Box<dyn Send + Sync + 'static>>> = RwLock::new(Vec::new());

pub struct Stash<T>(pub Wasm, pub PhantomData<T>);

impl<T> Stash<T>
where
    T: Send + Sync + 'static,
    for<'a> &'a T: IntoWasm,
{
    pub fn new(x: T) -> Self {
        let wasm = (&x).into_wasm();
        STASH.write().unwrap().push(Box::new(x));
        return Self(wasm, PhantomData);
    }
}

pub fn clear_stash() {
    STASH.write().unwrap().clear();
}

// IntoWasm impl
//

impl<T> ToWasm for Stash<T>
where
    T: Send + Sync + 'static,
    for<'a> &'a T: IntoWasm,
{
    fn to_wasm(&self) -> Wasm {
        Wasm(self.0 .0) // not sure why we don't just make Wasm Copy...
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
