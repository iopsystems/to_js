use crate::types::packed::U32Pair;
use crate::Wasm;

// Wrapping strategies for encoding Option<T> and Result<T> into a set of niche values.
// Each type opts in to one of these strategies based on what niche it has available, so
// these types are declared here and imported in the individual types that use them.

pub(crate) enum Niche {
    HighBitsNaN, // SNaN value signaling the condition in the high bits, with low bits=x
    LowBitsZero, // (ptr, len) pair with ptr=0 and len=x, encoding the info alongside a null pointer
}

impl Niche {
    pub(crate) fn value(self, x: u32) -> Wasm {
        const SIGNALING_NAN: u64 = 0xfff80000_00000000;
        match self {
            Self::HighBitsNaN => f64::from_bits(SIGNALING_NAN | x as u64).into(),
            Self::LowBitsZero => U32Pair([0, x]).into(),
        }
    }
}

pub(crate) trait HasNiche: Into<Wasm> {
    const N: Niche;
}
