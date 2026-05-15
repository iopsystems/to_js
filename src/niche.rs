use crate::types::packed::U32Pair;
use crate::ToWasm;
use crate::Wasm;

/// Encoding strategies for marking Option<T> and Result<T, E> variants using a set of niche values,
/// allowing us to overlay those types into the same 64 bits as the original value.
/// Each type that can be wrapped opts in to one of these strategies based on what niche it has
/// available, so these types are declared here and imported in the individual types that use them.
pub(crate) enum Niche {
    /// Signal that we're inside the niche by setting the high bits to those of a specific quiet NaN
    /// with a non-canonical mantissa payload, and encode the associated value in the low bits.
    HighBitsNaN,

    /// Signal that we're inside the niche by setting the low bits to 0x0001, and encode the associated
    /// value in the high bits. This is intended for use with types for which a byte-aligned pointer
    /// would typically be encoded into the low bits.
    LowBitsOne,
}

/// The 64-bit pattern is a quiet NaN (sign=1, exponent=all-1s, high mantissa bit set) with an
/// additional distinctive "face" payload baked into the high mantissa bits. No IEEE-754 arithmetic
/// operation produces a NaN with these specific mantissa bits — only an explicit `f64::from_bits`
/// could reproduce them — so the chance of a user-returned f64/f32 colliding with the niche is
/// effectively zero in practice. The JS side preserves f64 NaN payloads exactly across the wasm/JS
/// boundary and through typed-array overlays, so the upper 32 bits round-trip unchanged.
/// If you change this value, also update the matching constant in `lib.js`.
pub(crate) const NICHE_NAN_BITS: u64 = 0xfff8_face_0000_0000;

impl Niche {
    pub(crate) fn new(self, x: u32) -> Wasm {
        match self {
            Self::HighBitsNaN => f64::from_bits(NICHE_NAN_BITS | x as u64).to_wasm(),
            Self::LowBitsOne => U32Pair([1, x]).to_wasm(),
        }
    }
}

pub(crate) trait HasNiche {
    const N: Niche;
}

// Returning a reference to a value across the FFI boundary is treated the same as
// returning the value itself (which would be put in a stash, and the reference returned)
impl<T: HasNiche> HasNiche for &T {
    const N: Niche = T::N;
}
