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
        Stash(vec![x.value.value(), x.info.value()]).into()
    }
}

// HasNiche impl
//

impl HasNiche for Dynamic {
    const N: Niche = Niche::LowBitsOne;
}

// TypeInfo impl
//

impl_typeinfo!([Dynamic, ArrayType::F64, true, Transform::Dynamic]);
