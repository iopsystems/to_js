use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Transform};
use crate::Stash;
use crate::{ToWasm, Wasm};

// Represents a value to be serialized to JSON using serde.
pub struct Json(Stash<String>);

impl Json {
    pub fn new(x: &impl serde::Serialize) -> Json {
        let s = serde_json::to_string(x).expect("JSON serialization failed");
        Json(Stash::new(s))
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
