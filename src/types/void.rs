use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Transform};
use crate::ToWasm;
use crate::Wasm;

// From<...> for Wasm impl
//

impl From<()> for Wasm {
    fn from(_: ()) -> Self {
        Wasm(0f64)
    }
}

impl ToWasm for () {
    fn to_wasm(&self) -> Wasm {
        (*self).into()
    }
}

// HasNiche impl
//

impl HasNiche for () {
    const N: Niche = Niche::HighBitsNaN;
}

// TypeInfo impl
//

impl_typeinfo!([(), ArrayType::None, false, Transform::Void]);
