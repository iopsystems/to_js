use crate::typeinfo::{ArrayType, Transform, TypeInfo};
// use crate::{Stash, Wasm};

// From<...> for Wasm impl
//

// maybe we need another 'parallel' implementation where we explicitly declare all types allowed to be dynamic,
// without reference to wasm... but then how do we turn them into wasm and get the type info? we can't say : TypeInfo
// since that trait is not object-safe, since its method does not have a self parameter...
use crate::{Stash, Wasm};

pub struct Dynamic {
    value: Wasm,
    info: Wasm,
}

impl Dynamic {
    pub fn new<T>(x: T) -> Self
    where
        // Wasm: for<'a> From<&'a T>,
        Wasm: From<T>,
        T: TypeInfo,
    {
        Self {
            value: x.into(),
            info: T::type_info().into(),
        }
    }
}

impl From<Dynamic> for Wasm {
    fn from(x: Dynamic) -> Self {
        Stash(vec![x.value.value(), x.info.value()]).into()
    }
}

// impl From<Dynamic<Box<dyn Any>>> for Wasm
// where
//     Wasm: for<'a> From<&'a T>,
//     Wasm: From<Stash<T>>,
//     T: TypeInfo,
// {
//     fn from(x: Dynamic<Box<T>>) -> Self {
//         let wasm: Wasm = Stash(*x.0).into();
//         let info: Wasm = T::type_info().into();
//         Stash(vec![info.value(), wasm.value()]).into()
//     }
// }

// pub struct Dynamic<T>(pub T);

// impl<T> From<Dynamic<Box<T>>> for Wasm
// where
//     Wasm: for<'a> From<&'a T>,
//     Wasm: From<Stash<T>>,
//     T: TypeInfo,
// {
//     fn from(x: Dynamic<Box<T>>) -> Self {
//         let wasm: Wasm = Stash(*x.0).into();
//         let info: Wasm = T::type_info().into();
//         Stash(vec![info.value(), wasm.value()]).into()
//     }
// }

// maybe the dynamic itself is the thing inside the stash
// and it stores the wasm of its T
// and it stores the typeinfo

// pub struct Dynamic<T>(pub T);

// struct ItemAndThing<T>(Box<f64>, T);
// impl<T> From<Dynamic<T>> for Wasm
// where
//     Wasm: From<T>,
//     T: TypeInfo,
// {
//     fn from(x: Dynamic<T>) -> Self {
//         let wasm = Wasm::from(x.0);
//         let info: Wasm = T::type_info().into();
//         let item = Box::new(wasm.value());
//         let iat = ItemAndThing(item, x);
//         // Wasm::from(Stash(iat));
//         1234.0.into()
//     }
// }

// idea: return a single Dynamic as F64Array[wasm, typeinfo]. But how do we stash both this and the original item?

// HasNiche impl
//

// TypeInfo impl
//

// impl<T> TypeInfo for Dynamic {
//     fn type_info() -> Info {
//         Info::new(ArrayType::F64, false, Transform::Dynamic)
//     }
// }

impl_typeinfo!([Dynamic, ArrayType::F64, true, Transform::Dynamic]);

// what happens if you want to put a Stash<Vec<T>> into a Dynamic array? what gets stashed? or how do you put a vec into a dynamic otherwise?

// impl<T> From<T> for Dynamic
// where
//     // Wasm: for<'a> From<&'a T>,
//     Wasm: From<T>,
//     T: TypeInfo,
// {
//     fn from(x: T) -> Self {
//         Self {
//             wasm: x.into(),
//             info: T::type_info().into(),
//         }
//     }
// }
