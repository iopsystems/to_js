use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Transform, TypeInfo};
use crate::IntoWasm;
use crate::ToWasm;
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
        Stash<T>: IntoWasm + TypeInfo,
    {
        Self {
            value: Stash(x).into_wasm(),
            type_info: <Stash<T>>::type_info().into_wasm(),
        }
    }
}

// This type exists to resolve dispatch ambiguities between the vec/slice/box<[T] impls
// and the specific implementation we want for a slice/vec of Dynamic values.
pub struct DynamicArray(Vec<Dynamic>);

// From<...> for Wasm impl
//

impl IntoWasm for Dynamic {
    fn into_wasm(self) -> Wasm {
        Stash(vec![self.value.value(), self.type_info.value()].into_boxed_slice()).into_wasm()
    }
}

impl IntoWasm for DynamicArray {
    fn into_wasm(self) -> Wasm {
        Stash(
            self.0
                .into_iter()
                .flat_map(|x| [x.value.value(), x.type_info.value()])
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
        .into_wasm()
    }
}

// HasNiche impl
//

impl HasNiche for Dynamic {
    const N: Niche = Niche::LowBitsOne;
}

impl HasNiche for DynamicArray {
    const N: Niche = Niche::LowBitsOne;
}

// TypeInfo impl
//

impl_typeinfo! {
    [Dynamic,      ArrayType::F64, true, Transform::Dynamic],
    [DynamicArray, ArrayType::F64, true, Transform::DynamicArray],
}
