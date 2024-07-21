use crate::niche::HasNiche;
use crate::typeinfo::{Info, TypeInfo};
use crate::types::errorstring::ErrorString;
use crate::IntoWasm;
use crate::Wasm;

// We allow Option and Result as wrapper types, and they
// apportion the available space of niches between them
// to indicate None and Err return values, respectively.
// Option uses a 0 value in the free u32 to indicate None,
// while Result uses all other (nonzero) values to point
// to the error string.

// From<...> for Wasm impl
//

impl<T: HasNiche> IntoWasm for Option<T> {
    fn into_wasm(&self) -> Wasm {
        match self {
            Some(x) => x.into_wasm(),
            None => T::N.new(0),
        }
    }
}

impl<T: HasNiche, E: ErrorString> IntoWasm for Result<Option<T>, E> {
    fn into_wasm(&self) -> Self {
        x.transpose().into()
    }
}

impl<T: HasNiche, E: ErrorString> From<Result<T, E>> for Wasm {
    fn from(x: Result<T, E>) -> Self {
        match x {
            Ok(value) => value.into(),
            Err(e) => T::N.new(e.to_u32()),
        }
    }
}

impl<T: HasNiche, E: ErrorString> From<Option<Result<T, E>>> for Wasm {
    fn from(x: Option<Result<T, E>>) -> Self {
        match x {
            Some(value) => value.into(),
            None => None::<T>.into(),
        }
    }
}

// HasNiche impl
// (no impls since these types are the wrappers; they cannot themselves be wrapped)

// TypeInfo impl
//

impl<T: TypeInfo> TypeInfo for Option<T> {
    fn type_info() -> Info {
        T::type_info().option()
    }
}

impl<T: TypeInfo, E> TypeInfo for Result<T, E> {
    fn type_info() -> Info {
        T::type_info().result()
    }
}
