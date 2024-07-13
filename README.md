# To JS

A small package for simple WebAssembly-to-JavaScript exports for numbers, typed arrays, and strings. Supports returning `Option` and `Result` values, which can return `null` and throw an error, respectively. Typed arrays can be returned as views of Rust memory, avoiding data copies.

## Examples

```rust
use to_js::js;

#[js]
fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[js]
fn checked_add(a: u32, b: u32) -> Option<u32> {
    a.checked_add(b)
}

#[js]
fn str() -> &'static str {
    "Hello from a &'static str"
}

#[js]
fn slice() -> &'static [u32] {
    &[10, 20, 30]
}
```

Returning owned values is accomplished by wrapping them in a `Stash`, which ensures the value lives at least until the next call to a Rust function.

```rust
use to_js::Stash;

#[js]
fn string() -> Stash<String> {
    String::from("Hello from a String").into()
}

#[js]
fn vec(count_up_to: usize) -> Stash<Vec<usize>> {
    (1..=count_up_to).collect::<Vec<_>>().into()
}

#[js]
fn vec_result(count_up_to: usize) -> Result<Stash<Vec<usize>>, &'static str> {
    if count_up_to > 100 {
        return Err("I can't count that high.");
    }
    Ok(vec(count_up_to))
}
```

## Usage

```js
// Given a WebAssembly instance, return an object containing its #[js] exports.
// If the optional second argument is true, typed arrays will be copied out of
// WebAssembly memory before being returned, enhancing ease-of-use at the cost
// of extra data copies.
function toJs(instance, alwaysCopyData) {
  const buffer = instance.exports.memory.buffer;
  const view = new DataView(buffer);
  const ptr = view.getUint32(instance.exports.JS, true);
  const len = view.getUint32(instance.exports.JS + 4, true);
  const code = new TextDecoder().decode(buffer.slice(ptr, ptr + len));
  return import("data:text/javascript," + code).then((module) =>
    module.default(instance, alwaysCopyData)
  );
}

const rs = await WebAssembly.instantiateStreaming(
  fetch(url /* url to the compiled .wasm file */)
).then((results) => toJs(results.instance))

rs.add(2, 2) // => 4
rs.checked_add(2, 2) // => 4
rs.checked_add(2 ** 31, 2 ** 31) // => null
rs.str() // => "Hello from a &'static str"
rs.slice() // => Float64Array[10, 20, 30]
rs.string() // => "Hello from a String"
rs.vec(5) // => Uint32Array[1, 2, 3, 4, 5]
rs.vec_result(5) // => Uint32Array[1, 2, 3, 4, 5]
rs.vec_result(500) // => Error: I can't count that high.
```