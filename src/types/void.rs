use crate::typeinfo::{ArrayType, Transform};
use crate::niche::{Niche, HasNiche};
use crate::Wasm;

// Into<Wasm> impl via the From trait
//

impl From<()> for Wasm {
    fn from(_: ()) -> Self {
        Wasm(0f64)
    }
}

// Wrappable impl
//

impl HasNiche for () {
    const N: Niche = Niche::HighBitsNaN;
}

// TypeInfo impl
//

impl_typeinfo!([(), ArrayType::None, Transform::Void, false]);
