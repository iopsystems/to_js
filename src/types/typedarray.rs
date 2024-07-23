use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{Info, TypeInfo};
use crate::types::number::Number;
use crate::types::packed::U32Pair;
use crate::IntoWasm;
use crate::ToWasm;
use crate::Wasm;

// From<...> for Wasm impl
//

// TODO: impl for [T] and Box<T> rather than &[T] and &Box<T> once we are not deferring to the From Wasm impls
// todo: should these be ToWasm or IntoWasm?
// We use IntoWasm for &mut references, but this all feels a bit sus since they're references
impl<T: Number> ToWasm for &[T] {
    fn to_wasm(&self) -> Wasm {
        U32Pair([self.as_ptr() as u32, self.len() as u32]).into_wasm()
    }
}

impl<T: Number> IntoWasm for &mut [T] {
    fn into_wasm(self) -> Wasm {
        U32Pair([self.as_mut_ptr() as u32, self.len() as u32]).into_wasm()
    }
}

impl<T: Number> ToWasm for Box<[T]> {
    fn to_wasm(&self) -> Wasm {
        U32Pair([self.as_ptr() as u32, self.len() as u32]).into_wasm()
    }
}

impl<T: Number> IntoWasm for &mut Box<[T]> {
    fn into_wasm(self) -> Wasm {
        U32Pair([self.as_mut_ptr() as u32, self.len() as u32]).into_wasm()
    }
}

// HasNiche impl
//

impl<T: Number> HasNiche for &[T] {
    const N: Niche = Niche::LowBitsOne;
}

impl<T: Number> HasNiche for &mut [T] {
    const N: Niche = Niche::LowBitsOne;
}

impl<T: Number> HasNiche for Box<[T]> {
    const N: Niche = Niche::LowBitsOne;
}

impl<T: Number> HasNiche for &mut Box<[T]> {
    const N: Niche = Niche::LowBitsOne;
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

impl<T: Number + TypeInfo> TypeInfo for Box<[T]> {
    fn type_info() -> Info {
        T::type_info().array().identity_transform()
    }
}

impl<T: Number + TypeInfo> TypeInfo for &mut Box<[T]> {
    fn type_info() -> Info {
        T::type_info().array().identity_transform()
    }
}
