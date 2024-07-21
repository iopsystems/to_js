use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Transform};
use crate::IntoWasm;
use crate::Wasm;

// From<...> for Wasm impl
//

impl IntoWasm for bool {
    fn into_wasm(&self) -> Wasm {
        Wasm(*self as u8 as f64)
    }
}

// HasNiche impl
//

impl HasNiche for bool {
    const N: Niche = Niche::HighBitsNaN;
}

// TypeInfo impl
//

impl_typeinfo!([bool, ArrayType::None, false, Transform::Bool]);
