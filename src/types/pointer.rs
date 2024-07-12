use crate::typeinfo::{ArrayType, Info, Transform, TypeInfo};
use crate::niche::{Niche, HasNiche};
use crate::Wasm;

// Into<Wasm> impl via the From trait
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

// Wrappable impl
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
        Info::new(ArrayType::None, Transform::Identity, false)
    }
}

impl<T> TypeInfo for *mut T {
    fn type_info() -> Info {
        Info::new(ArrayType::None, Transform::Identity, false)
    }
}
