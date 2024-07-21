use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Transform, TypeInfo};
use crate::{Stash, Wasm};

// From<...> for Wasm impl
//

pub struct Dynamic {
    value: Wasm,
    info: Wasm,
}

impl Dynamic {
    pub fn new<T>(x: T) -> Self
    where
        Stash<T>: Into<Wasm> + TypeInfo,
    {
        Self {
            value: Stash(x).into(),
            info: <Stash<T>>::type_info().into(),
        }
    }
}

// From<...> for Wasm impl
//

impl From<Dynamic> for Wasm {
    fn from(x: Dynamic) -> Self {
        // A single element is encoded the same as an array, but has different TypeInfo so that
        // it's returned as a single element on the JS side rather than a one-element array.
        DynamicArray(vec![x]).into()
    }
}

pub struct DynamicArray(Vec<Dynamic>);

impl From<Vec<Dynamic>> for DynamicArray {
    fn from(x: Vec<Dynamic>) -> Self {
        Self(x)
    }
}

impl From<DynamicArray> for Wasm {
    fn from(x: DynamicArray) -> Self {
        Stash(
            x.0.into_iter()
                .flat_map(|x| [x.value.value(), x.info.value()])
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
        .into()
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

// impl HasNiche for Vec<Dynamic> {
//     const N: Niche = Niche::LowBitsOne;
// }

// TypeInfo impl
//

impl_typeinfo! {
    [Dynamic, ArrayType::F64, true, Transform::Dynamic],
    [DynamicArray, ArrayType::F64, true, Transform::DynamicArray],
}
