use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{ArrayType, Info, Transform, TypeInfo};
use crate::ToWasm;
use crate::Wasm;

// From<...> for Wasm impl
//

impl<T> From<*const T> for Wasm {
    fn from(x: *const T) -> Self {
        (x as u32).into()
    }
}

impl<T> From<*mut T> for Wasm {
    fn from(x: *mut T) -> Self {
        (x as u32).into()
    }
}

impl<T> ToWasm for *const T {
    fn to_wasm(&self) -> Wasm {
        (*self).into()
    }
}

impl<T> ToWasm for *mut T {
    fn to_wasm(&self) -> Wasm {
        (*self).into()
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
