# To JS

A small package for simple WebAssembly exports for numbers, typed arrays, and strings. Supports returning `Option` and `Result` values, which return `null` and throw an error, respectively. Typed arrays can be returned as views of Rust memory, avoiding copies of their data.

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
fn range(len: usize) -> Result<Stash<Vec<usize>>, &'static str> {
    if len > 100 {
        return Err("I can't count that high.");
    }
    let mut v = vec![0; len];
    for (i, v) in v.iter_mut().enumerate() {
        *v = i;
    }
    Ok(v.into())
}
```

## Usage

```js
// Given a WebAssembly instance, return an object containing its #[js] exports.
// If the optional second argument is true, typed arrays will be copied out of
// the WebAssembly memory space upon return, enhancing ease-of-use at the cost
// of performing the data copies.
function toJs(instance, alwaysCopyData) {
  const buffer = instance.exports.memory.buffer;
  const view = new DataView(buffer);
  const ptr = view.getUint32(instance.exports.JS, true);
  const len = view.getUint32(instance.exports.JS + 4, true);
  const code = new TextDecoder().decode(buffer.slice(ptr + 3, ptr + len));
  return import(code).then((module) =>
    module.default(instance, alwaysCopyData)
  );
}

const rs = await WebAssembly.instantiateStreaming(
  fetch(await file.url())
).then((results) => toJs(results.instance))

rs.add(2, 2) // -> 4
rs.checked_add(2, 2) // -> 4
rs.checked_add(2 ** 31, 2 ** 31) // -> null
rs.str() // -> "Hello from a &'static str"
rs.slice() // -> Float64Array[10, 20, 30]
rs.string() // -> "Hello from a String"
rs.range(5) // -> Uint32Array[0, 1, 2, 3, 4]
```