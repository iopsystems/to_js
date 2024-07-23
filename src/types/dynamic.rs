use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Transform, TypeInfo};
use crate::{IntoWasm, ToWasm};
use crate::{Stash, Wasm};

pub struct Dynamic {
    value: Wasm,
    type_info: Wasm,
}

impl Dynamic {
    /// Construct a new Dynamic value. All Dynamic construction goes through
    /// this function, ensuring that dynamic values are stashed before being
    /// returned across the WebAssembly FFI boundary.
    pub fn new<T>(x: T) -> Self
    where
        T: ToWasm + Send + Sync + 'static,
        Stash<T>: TypeInfo,
    {
        Self {
            value: Stash::new(x).to_wasm(),
            type_info: <Stash<T>>::type_info().to_wasm(),
        }
    }
}

// ToWasm impl
//

impl ToWasm for Dynamic {
    fn to_wasm(&self) -> Wasm {
        Stash::new(vec![self.value.value(), self.type_info.value()].into_boxed_slice()).to_wasm()
    }
}

impl ToWasm for &[Dynamic] {
    fn to_wasm(&self) -> Wasm {
        Stash::new(
            self.iter()
                .flat_map(|x| [x.value.value(), x.type_info.value()])
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
        .to_wasm()
    }
}

impl ToWasm for &Box<[Dynamic]> {
    fn to_wasm(&self) -> Wasm {
        self[..].into_wasm()
    }
}

// HasNiche impl
//

impl HasNiche for Dynamic {
    const N: Niche = Niche::LowBitsOne;
}

impl HasNiche for &[Dynamic] {
    const N: Niche = Niche::LowBitsOne;
}

impl HasNiche for &Box<[Dynamic]> {
    const N: Niche = Niche::LowBitsOne;
}

// TypeInfo impl
//

impl_typeinfo! {
    [Dynamic,         ArrayType::F64, true, Transform::Dynamic],
    [&[Dynamic],      ArrayType::F64, true, Transform::DynamicArray],
    [&Box<[Dynamic]>, ArrayType::F64, true, Transform::DynamicArray],
}
