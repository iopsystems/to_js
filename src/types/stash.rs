use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{Info, TypeInfo};
use crate::{IntoWasm, ToWasm, Wasm};
use std::marker::PhantomData;
use std::sync::RwLock;

// Global stash to keep values alive across FFI boundary until the next FFI call,
// storing them in a vector of type-erased boxes, to be dropped when the next value is put in.
static STASH: RwLock<Vec<Box<dyn Send + Sync + 'static>>> = RwLock::new(Vec::new());

pub struct Stash<T>(Wasm, PhantomData<T>);

// This constructor eagerly writes the wasm representation of `x` into STASH.
// We do this here rather than in Stash::to_wasm since it allows Stash
// to implement ToWasm, which does not require ownership, but
// is a semantically valid operation while STASH owns the value.
impl<T> Stash<T>
where
    T: Send + Sync + 'static,
    for<'a> &'a T: IntoWasm,
{
    pub fn new(x: T) -> Stash<T> {
        let wasm = (&x).into_wasm();
        STASH.write().unwrap().push(Box::new(x));
        return Stash(wasm, PhantomData);
    }
}

pub fn clear_stash() {
    STASH.write().unwrap().clear();
}

// ToWasm impl
//

impl<T> ToWasm for Stash<T> {
    fn to_wasm(&self) -> Wasm {
        self.0.clone()
    }
}

// HasNiche impl
//

impl<T> HasNiche for Stash<T>
where
    for<'a> &'a T: HasNiche,
{
    const N: Niche = <&T>::N;
}

// TypeInfo impl
//

impl<T> TypeInfo for Stash<T>
where
    for<'a> &'a T: TypeInfo,
{
    fn type_info() -> Info {
        <&T>::type_info()
    }
}
