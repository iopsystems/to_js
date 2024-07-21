use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{Info, TypeInfo};
use crate::types::number::Number;
use crate::Wasm;

// From<...> for Wasm impl
//

impl<T: Number> From<&Vec<T>> for Wasm {
    fn from(x: &Vec<T>) -> Self {
        x.as_slice().into()
    }
}

// This would enable auto-stashing but conflicts with the blanket From<X: Copy> impl in lib.rs
// impl<T: Number> From<Vec<T>> for Wasm {
//     fn from(x: Vec<T>) -> Self {
//         Stash(x).into()
//     }
// }

// HasNiche impl
//

impl<T: Number> HasNiche for &Vec<T> {
    const N: Niche = Niche::LowBitsOne;
}

// TypeInfo impl
//

impl<T: Number + TypeInfo> TypeInfo for &Vec<T> {
    fn type_info() -> Info {
        <&[T]>::type_info()
    }
}
