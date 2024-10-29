use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Transform};
use crate::{ToWasm, Wasm};
use miniserde::{json, Serialize};

// Represents a value to be serialized to JSON using miniserde.
pub struct Json(String);

impl Json {
    pub fn new(x: impl Serialize) -> Json {
        Json(json::to_string(&x))
    }
}

// ToWasm impl
//

impl ToWasm for Json {
    fn to_wasm(&self) -> Wasm {
        self.0.to_wasm()
    }
}

// HasNiche impl

impl HasNiche for Json {
    const N: Niche = String::N;
}

// TypeInfo impl
//

impl_typeinfo! {
    [Json,  ArrayType::U8, true, Transform::Json],
}
