use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Info, Transform, TypeInfo};
use crate::{ToWasm, Wasm};

// ToWasm impl
//

impl<T> ToWasm for *const T {
    fn to_wasm(&self) -> Wasm {
        (*self as u32).to_wasm()
    }
}

impl<T> ToWasm for *mut T {
    fn to_wasm(&self) -> Wasm {
        (*self as u32).to_wasm()
    }
}

// HasNiche impl
//

impl<T> HasNiche for *const T {
    const N: Niche = Niche::HighBitsNaN;
}

impl<T> HasNiche for *mut T {
    const N: Niche = Niche::HighBitsNaN;
}

// TypeInfo impl
//

impl<T> TypeInfo for *const T {
    fn type_info() -> Info {
        Info::new(ArrayType::None, false, Transform::Identity)
    }
}

impl<T> TypeInfo for *mut T {
    fn type_info() -> Info {
        Info::new(ArrayType::None, false, Transform::Identity)
    }
}
