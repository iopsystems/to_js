use crate::niche::HasNiche;
use crate::typeinfo::{Info, TypeInfo};
use crate::types::errorstring::ErrorString;
use crate::ToWasm;
use crate::Wasm;

// We allow Option and Result as wrapper types, and they
// apportion the available space of niches between them
// to indicate None and Err return values, respectively.
// Option uses a 0 value in the free u32 to indicate None,
// while Result uses all other (nonzero) values to point
// to the error string.

// ToWasm impl
//

impl<T: HasNiche + ToWasm> ToWasm for Option<T> {
    fn to_wasm(&self) -> Wasm {
        match self {
            Some(value) => value.to_wasm(),
            None => T::N.new(0),
        }
    }
}

impl<T: HasNiche + ToWasm, E: ErrorString> ToWasm for Result<Option<T>, E> {
    fn to_wasm(&self) -> Wasm {
        match self {
            Ok(Some(value)) => Ok::<&T, &E>(value).to_wasm(),
            Ok(None) => None::<T>.to_wasm(),
            Err(e) => Err::<&T, &E>(e).to_wasm(),
        }
    }
}

impl<T: HasNiche + ToWasm, E: ErrorString> ToWasm for Result<T, E> {
    fn to_wasm(&self) -> Wasm {
        match self {
            Ok(value) => value.to_wasm(),
            Err(e) => T::N.new(e.to_u32()),
        }
    }
}

impl<T: HasNiche + ToWasm, E: ErrorString> ToWasm for Option<Result<T, E>> {
    fn to_wasm(&self) -> Wasm {
        match self {
            Some(value) => value.to_wasm(),
            None => None::<T>.to_wasm(),
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
