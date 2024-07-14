use crate::typeinfo::{ArrayType, Info, Transform, TypeInfo};
use crate::Wasm;

// From<...> for Wasm impl
//

pub struct Dynamic<T>(T);

impl<T> From<Dynamic<T>> for Wasm
where
    for<'a> &'a T: Into<Wasm>,
    T: TypeInfo,
{
    fn from(x: Dynamic<T>) -> Self {
        let wasm = (&x.0).into();
        let info = T::type_info();
        info.to_u32().into()
    }
}

// HasNiche impl
//

// TypeInfo impl
//

impl<T> TypeInfo for Dynamic<T> {
    fn type_info() -> Info {
        Info::new(ArrayType::None, false, Transform::Dynamic)
    }
}

// what happens if you want to put a Stash<Vec<T>> into a Dynamic array? what gets stashed? or how do you put a vec into a dynamic otherwise?
