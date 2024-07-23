use crate::types::packed::U32Pair;
use crate::IntoWasm;
use crate::Wasm;

/// Encoding strategies for marking Option<T> and Result<T> variants using a set of niche values,
/// allowing us to overlay those types into the same 64 bits as the original value.
/// Each type that can be wrapped opts in to one of these strategies based on what niche it has
/// available, so these types are declared here and imported in the individual types that use them.
pub(crate) enum Niche {
    /// Signal that we're inside the niche by setting the high bits to those of a signaling NaN,
    /// and encode the associated value in the low bits.
    HighBitsNaN,

    /// Signal that we're inside the niche by setting the low bits to 0x0001, and encode the associated
    /// value in the high bits. This is intended for use with types for which a byte-aligned pointer
    /// would typically be encoded into the low bits.
    LowBitsOne,
}

impl Niche {
    pub(crate) fn new(self, x: u32) -> Wasm {
        const SIGNALING_NAN: u64 = 0xfff80000_00000000;
        match self {
            Self::HighBitsNaN => f64::from_bits(SIGNALING_NAN | x as u64).into_wasm(),
            Self::LowBitsOne => U32Pair([1, x]).into_wasm(),
        }
    }
}

pub(crate) trait HasNiche {
    const N: Niche;
}
