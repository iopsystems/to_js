use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Transform};
use crate::{ToWasm, Wasm};

// ToWasm impl
//

impl ToWasm for () {
    fn to_wasm(&self) -> Wasm {
        Wasm(0f64)
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
