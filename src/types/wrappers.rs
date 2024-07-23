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

impl<T: HasNiche + IntoWasm> IntoWasm for Option<T> {
    fn into_wasm(self) -> Wasm {
        match self {
            Some(value) => value.into_wasm(),
            None => T::N.new(0),
        }
    }
}

impl<T: HasNiche + IntoWasm, E: ErrorString> IntoWasm for Result<Option<T>, E> {
    fn into_wasm(self) -> Wasm {
        match self {
            Ok(Some(value)) => Ok::<T, E>(value).into_wasm(),
            Ok(None) => None::<T>.into_wasm(),
            Err(e) => Err::<T, E>(e).into_wasm(),
        }
    }
}

impl<T: HasNiche + IntoWasm, E: ErrorString> IntoWasm for Result<T, E> {
    fn into_wasm(self) -> Wasm {
        match self {
            Ok(value) => value.into_wasm(),
            Err(e) => T::N.new(e.to_u32()),
        }
    }
}

impl<T: HasNiche + IntoWasm, E: ErrorString> IntoWasm for Option<Result<T, E>> {
    fn into_wasm(self) -> Wasm {
        match self {
            Some(value) => value.into_wasm(),
            None => None::<T>.into_wasm(),
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
