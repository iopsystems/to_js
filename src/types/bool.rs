use crate::typeinfo::{ArrayType, Transform};
use crate::niche::{Niche, HasNiche};
use crate::Wasm;

// From<...> for Wasm impl
//

impl From<bool> for Wasm {
    fn from(x: bool) -> Self {
        Wasm(x as u8 as f64)
    }
}

// HasNiche impl
//

impl HasNiche for bool {
    const N: Niche = Niche::HighBitsNaN;
}

// TypeInfo impl
//

impl_typeinfo!([bool, ArrayType::None, Transform::Bool, false]);
