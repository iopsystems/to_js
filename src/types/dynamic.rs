use crate::typeinfo::{ArrayType, Info, Transform, TypeInfo};
use crate::Wasm;

// From<...> for Wasm impl
//

pub struct Dynamic<T>(pub T);

impl<T> From<Dynamic<T>> for Wasm
where
    Wasm: From<T>,
    T: TypeInfo,
{
    fn from(x: Dynamic<T>) -> Self {
        // let wasm = Wasm::from(x.0);
        // let wasm = Stash(x).into();
        // let info = T::type_info();
        // [info, wasm].into()
        123.4.into()
    }
}

// idea: return a single Dynamic as F64Array[wasm, typeinfo]. But how do we stash both this and the original item?

// HasNiche impl
//

// TypeInfo impl
//

impl<T> TypeInfo for Dynamic<T> {
    fn type_info() -> Info {
        Info::new(ArrayType::F64, false, Transform::Dynamic)
    }
}

// what happens if you want to put a Stash<Vec<T>> into a Dynamic array? what gets stashed? or how do you put a vec into a dynamic otherwise?
