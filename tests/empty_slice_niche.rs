// Regression tests for the empty-slice / niche collision fix in typedarray.rs.
//
// Before the fix, an empty `&[u8]` used Rust's dangling pointer (= `align_of::<u8>()` = 1),
// which collided exactly with the `LowBitsOne` niche encoding `(low=1, high=0)`. That made
// `Option::Some(&[]) as Option<&[u8]>` indistinguishable from `None`, so JS saw `null`.
//
// These tests run on the host target (`cargo test`) — the bit-level encoding logic is
// platform-agnostic, so we can verify the fix without spinning up a wasm runtime.

use to_js::ToWasm;

fn bits<T: ToWasm>(value: T) -> u64 {
    value.to_wasm().value().to_bits()
}

#[test]
fn empty_u8_slice_distinct_from_none() {
    let empty: &[u8] = &[];
    assert_ne!(
        bits(Some(empty)),
        bits(None::<&[u8]>),
        "Some(&[]) and None must encode to different bit patterns"
    );
}

#[test]
fn empty_i8_slice_distinct_from_none() {
    // i8 has alignment 1 too, so it hits the same dangling-pointer collision.
    let empty: &[i8] = &[];
    assert_ne!(bits(Some(empty)), bits(None::<&[i8]>));
}

#[test]
fn empty_str_from_dangling_string_distinct_from_none() {
    // String::new() does not allocate; its as_str() has the dangling pointer (= 1).
    // This is the most direct trigger for the pre-fix collision.
    let s = String::new();
    let empty: &str = s.as_str();
    assert_ne!(bits(Some(empty)), bits(None::<&str>));
}

#[test]
fn empty_vec_distinct_from_none() {
    let v: Vec<u8> = Vec::new();
    assert_ne!(bits(Some(&v)), bits(None::<&Vec<u8>>));
}

#[test]
fn empty_slice_encodes_as_zero() {
    // After the fix, an empty slice always has ptr=0 and len=0, so the f64 bit pattern is 0.
    let empty: &[u8] = &[];
    assert_eq!(bits(empty), 0);
}

#[test]
fn none_slice_encodes_as_low_bits_one() {
    // None for LowBitsOne is U32Pair([1, 0]); as a u64 that's 1 (little-endian).
    assert_eq!(bits(None::<&[u8]>), 1);
}

#[test]
fn non_empty_slice_encodes_with_real_pointer() {
    // Sanity: a non-empty slice's encoding has a non-zero pointer in the low 32 bits
    // and the length in the high 32 bits, so it agrees with the JS-side decoder.
    let data: &[u8] = &[10, 20, 30];
    let encoded = bits(data);
    let ptr = encoded as u32;
    let len = (encoded >> 32) as u32;
    assert_ne!(ptr, 0);
    assert_eq!(len, 3);
}
