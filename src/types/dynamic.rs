use std::collections::BTreeMap;

use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Transform, TypeInfo};
use crate::{IntoWasm, Stash, ToWasm, Wasm};

#[derive(Clone, Copy)]
pub struct Dynamic {
    value: Wasm,
    type_info: Wasm,
}

impl Dynamic {
    /// Construct a new Dynamic value. All Dynamic construction goes through
    /// this function, ensuring that dynamic values are stashed before being
    /// returned across the FFI boundary. The bounds on T are the the those
    /// needed for stashable values.
    pub fn new<T>(x: T) -> Self
    where
        T: Send + Sync + 'static,
        for<'a> &'a T: IntoWasm,
        Stash<T>: TypeInfo,
    {
        Self {
            value: Stash::new(x).to_wasm(),
            type_info: <Stash<T>>::type_info().to_wasm(),
        }
    }
}

// Convenience constructors
//

impl<const N: usize> From<[Dynamic; N]> for Dynamic {
    fn from(x: [Dynamic; N]) -> Self {
        Dynamic::new(x.to_vec().into_boxed_slice())
    }
}

impl From<Box<[Dynamic]>> for Dynamic {
    fn from(x: Box<[Dynamic]>) -> Self {
        Dynamic::new(x)
    }
}

impl From<BTreeMap<&'static str, Dynamic>> for Dynamic {
    fn from(x: BTreeMap<&'static str, Dynamic>) -> Self {
        Dynamic::new(x)
    }
}

// ToWasm impl
//

impl ToWasm for Dynamic {
    fn to_wasm(&self) -> Wasm {
        [*self].into_wasm()
    }
}

impl ToWasm for &[Dynamic] {
    fn to_wasm(&self) -> Wasm {
        Stash::new(
            self.iter()
                .flat_map(|x| [x.value.value(), x.type_info.value()])
                .collect::<Box<[f64]>>(),
        )
        .into_wasm()
    }
}

impl ToWasm for &Box<[Dynamic]> {
    fn to_wasm(&self) -> Wasm {
        self[..].into_wasm()
    }
}

impl ToWasm for &BTreeMap<&'static str, Dynamic> {
    fn to_wasm(&self) -> Wasm {
        self.iter()
            .flat_map(|(k, v)| [Dynamic::new(*k), *v])
            .collect::<Box<[Dynamic]>>()
            .into_wasm()
    }
}

// HasNiche impl
// (Dynamics and composites of Dynamics are encoded as (ptr, len) via &Box<[T]>)

impl HasNiche for Dynamic {
    const N: Niche = Niche::LowBitsOne;
}

impl HasNiche for &[Dynamic] {
    const N: Niche = Niche::LowBitsOne;
}

impl HasNiche for &Box<[Dynamic]> {
    const N: Niche = Niche::LowBitsOne;
}

impl HasNiche for &BTreeMap<&'static str, Dynamic> {
    const N: Niche = Niche::LowBitsOne;
}

// TypeInfo impl
//

impl_typeinfo! {
    [Dynamic,                          ArrayType::F64, true, Transform::Dynamic],
    [&[Dynamic],                       ArrayType::F64, true, Transform::DynamicArray],
    [&Box<[Dynamic]>,                  ArrayType::F64, true, Transform::DynamicArray],
    [&BTreeMap<&'static str, Dynamic>, ArrayType::F64, true, Transform::DynamicObject],
}
