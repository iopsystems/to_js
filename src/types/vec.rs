use crate::niche::{HasNiche, Niche};
use crate::typeinfo::{Info, TypeInfo};
use crate::types::number::Number;
use crate::{ToWasm, Wasm};

// ToWasm impl
//

impl<T> ToWasm for &Vec<T>
where
    for<'a> &'a [T]: ToWasm,
{
    fn to_wasm(&self) -> Wasm {
        self.as_slice().to_wasm()
    }
}

// HasNiche impl
//

impl<T: Number> HasNiche for &Vec<T> {
    const N: Niche = Niche::LowBitsOne;
}

// TypeInfo impl
//

impl<T: TypeInfo + Number> TypeInfo for &Vec<T> {
    fn type_info() -> Info {
        <&[T]>::type_info()
    }
}
