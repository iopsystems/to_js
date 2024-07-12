use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{Info, TypeInfo};
use crate::types::number::Number;
use crate::types::packed::U32Pair;
use crate::Wasm;

// From<...> for Wasm impl
//

impl<T: Number> From<&[T]> for Wasm {
    fn from(x: &[T]) -> Self {
        U32Pair([x.as_ptr() as u32, x.len() as u32]).into()
    }
}

impl<T: Number> From<&mut [T]> for Wasm {
    fn from(x: &mut [T]) -> Self {
        U32Pair([x.as_mut_ptr() as u32, x.len() as u32]).into()
    }
}

impl<T: Number> From<&Box<[T]>> for Wasm {
    fn from(x: &Box<[T]>) -> Self {
        U32Pair([x.as_ptr() as u32, x.len() as u32]).into()
    }
}

impl<T: Number> From<&mut Box<[T]>> for Wasm {
    fn from(x: &mut Box<[T]>) -> Self {
        U32Pair([x.as_mut_ptr() as u32, x.len() as u32]).into()
    }
}

// HasNiche impl
//

impl<T: Number> HasNiche for &[T] {
    const N: Niche = Niche::LowBitsZero;
}

impl<T: Number> HasNiche for &mut [T] {
    const N: Niche = Niche::LowBitsZero;
}

impl<T: Number> HasNiche for &Box<[T]> {
    const N: Niche = Niche::LowBitsZero;
}

impl<T: Number> HasNiche for &mut Box<[T]> {
    const N: Niche = Niche::LowBitsZero;
}

// TypeInfo impl
//

impl<T: Number + TypeInfo> TypeInfo for &[T] {
    fn type_info() -> Info {
        T::type_info().array().identity_transform()
    }
}

impl<T: Number + TypeInfo> TypeInfo for &mut [T] {
    fn type_info() -> Info {
        T::type_info().array().identity_transform()
    }
}

impl<T: Number + TypeInfo> TypeInfo for &Box<[T]> {
    fn type_info() -> Info {
        T::type_info().array().identity_transform()
    }
}

impl<T: Number + TypeInfo> TypeInfo for &mut Box<[T]> {
    fn type_info() -> Info {
        T::type_info().array().identity_transform()
    }
}
