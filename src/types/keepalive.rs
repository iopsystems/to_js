use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{Info, TypeInfo};
use crate::{IntoWasm, ToWasm, Wasm};
use std::marker::PhantomData;
use std::sync::RwLock;

/// Global stash to keep values alive across FFI boundary until the next FFI call,
/// storing them in a vector of type-erased boxes, to be dropped when the next value is put in.
/// This is a Vec so more than one value can be stashed during the execution of a single JS call.
static KEEPALIVE: RwLock<Vec<Box<dyn Send + Sync + 'static>>> = RwLock::new(Vec::new());

pub struct KeepAlive<T>(Wasm, PhantomData<T>);

// This constructor eagerly writes the wasm representation of `x` into KEEPALIVE.
// We do this here rather than in Stash::to_wasm since it allows Stash
// to implement ToWasm, which does not require ownership, but
// is a semantically valid operation while KEEPALIVE owns the value.
impl<T> KeepAlive<T>
where
    T: Send + Sync + 'static,
    for<'a> &'a T: IntoWasm,
{
    pub fn new(x: T) -> KeepAlive<T> {
        let wasm = (&x).into_wasm();
        KEEPALIVE.write().unwrap().push(Box::new(x));
        return KeepAlive(wasm, PhantomData);
    }
}

pub fn clear_keepalive() {
    KEEPALIVE.write().unwrap().clear();
}

// ToWasm impl
//

impl<T> ToWasm for KeepAlive<T> {
    fn to_wasm(&self) -> Wasm {
        self.0.clone()
    }
}

// HasNiche impl
//

impl<T> HasNiche for KeepAlive<T>
where
    for<'a> &'a T: HasNiche,
{
    const N: Niche = <&T>::N;
}

// TypeInfo impl
//

impl<T> TypeInfo for KeepAlive<T>
where
    for<'a> &'a T: TypeInfo,
{
    fn type_info() -> Info {
        <&T>::type_info()
    }
}
