use crate::typeinfo::{ArrayType, Info, Transform, TypeInfo};
use crate::Wasm;

// From<...> for Wasm impl
//

pub struct Dynamic<T>(pub T);

impl<T> Dynamic<T>
where
    T: Into<Wasm>,
{
    pub fn new(x: T) -> Self {
        Dynamic(x)
    }
}

impl<T> From<Dynamic<T>> for Wasm
where
    T: Into<Wasm>,
{
    fn from(x: Dynamic<T>) -> Self {
        x.0.into()
    }
}

// HasNiche impl
//

// TypeInfo impl
//

impl<T> TypeInfo for Dynamic<T>
where
    T: Into<Wasm>,
{
    fn type_info() -> Info {
        Info::new(ArrayType::None, false, Transform::Dynamic)
    }
}

// what happens if you want to put a Stash<Vec<T>> into a Dynamic array? what gets stashed? or how do you put a vec into a dynamic otherwise?
