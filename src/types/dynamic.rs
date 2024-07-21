use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Transform, TypeInfo};
use crate::{Stash, Wasm};

pub struct Dynamic {
    value: Wasm,
    info: Wasm,
}

impl Dynamic {
    /// Construct a new Dynamic value. All Dynamic construction goes through
    /// this function, ensuring that the values are stashed before being
    /// returned across the WebAssembly FFI boundary.
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

impl<T> From<T> for Dynamic
where
    Stash<T>: Into<Wasm> + TypeInfo,
{
    fn from(x: T) -> Self {
        Self::new(x)
    }
}

// This type exists to resolve dispatch ambiguities between the vec/slice/box<[T] impls
// and the specific implementation we want for a slice/vec of Dynamic values.
pub struct DynamicArray(Vec<Dynamic>);

impl<T> From<T> for DynamicArray
where
    T: Into<Vec<Dynamic>>,
{
    fn from(x: T) -> Self {
        Self(x.into())
    }
}

// From<...> for Wasm impl
//

impl From<Dynamic> for Wasm {
    fn from(x: Dynamic) -> Self {
        // A Dynamic is encoded like a DynamicArray, but has different TypeInfo so that
        // it can be returned as a single element rather than an Array on the JS side.
        DynamicArray(vec![x]).into()
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

// TypeInfo impl
//

impl_typeinfo! {
    [Dynamic,      ArrayType::F64, true, Transform::Dynamic],
    [DynamicArray, ArrayType::F64, true, Transform::DynamicArray],
}
