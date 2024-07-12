use crate::typeinfo::{Info, TypeInfo};
use crate::types::number::Number;
use crate::niche::{Niche, HasNiche};
use crate::Wasm;

// Into<Wasm> impl via the From trait
//

impl<T: Number> From<&Vec<T>> for Wasm {
    fn from(x: &Vec<T>) -> Self {
        x.as_slice().into()
    }
}

// Wrappable impl
//

impl<T: Number> HasNiche for &Vec<T> {
    const N: Niche = Niche::LowBitsZero;
}

// TypeInfo impl
//

impl<T: Number + TypeInfo> TypeInfo for &Vec<T> {
    fn type_info() -> Info {
        <&[T]>::type_info()
    }
}
