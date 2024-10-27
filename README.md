# To JS

A small package for simple WebAssembly-to-JavaScript exports for numbers, typed arrays, and strings. Supports returning `Option`s (as `null`) and `Result`s (throwing an exception). Typed arrays can be returned as views of Rust memory, avoiding data copies.

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

Returning owned values is accomplished by wrapping them in a `Stash`, which ensures the value lives until the next FFI call from JS to a Rust function.

```rust
use to_js::{stash, Stash};

#[js]
fn string() -> Stash<String> {
    stash(String::from("Hello from a String"))
}

#[js]
fn vec(count_up_to: usize) -> Stash<Vec<usize>> {
    stash((1..=count_up_to).collect::<Vec<_>>())
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
// If the optional second argument is true, typed arrays (including ones that
// were stashed) will be copied out of WebAssembly memory before being returned,
// enhancing ease-of-use at the cost of extra data copies.
function toJs(instance, alwaysCopyData) {
  const view = new DataView(instance.exports.memory.buffer);
  const ptr = view.getUint32(instance.exports.JS, true);
  const len = view.getUint32(instance.exports.JS + 4, true);
  const code = new TextDecoder().decode(view.buffer.slice(ptr, ptr + len));
  return import(encodeURI("data:text/javascript," + code)).then((module) =>
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

## Dynamic return types

As an experimental feature you can return dynamically-typed values using the `Dynamic` type:

```rust
use to_js::Dynamic;

#[js]
fn string_or_int(x: u32) -> Result<Dynamic, &'static str> {
    match x {
        0..10 => Ok(Dynamic::new(String::from("hi from a String"))),
        10..100 => Ok(Dynamic::new(123)),
        _ => Err("no dynamic for you!"),
    }
}
```

You can can return dynamic arrays, which are represented on the other side of the FFI boundary as plain [JavaScript arrays](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array):

```rust
#[js]
fn dynamic_array() -> Stash<Box<[Dynamic]>> {
    Stash::new(
        [
            Dynamic::new("hi"),
            Dynamic::new(Some(123.0)),
            Dynamic::new::<Option<&'static str>>(None),
        ].into()
    )
}
```

And you can return dynamic objects, which are represented on the other side of the FFI boundary as [JavaScript objects](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object):

```rust
fn dynamic_object(x: u32) -> Dynamic {
    vec![("key", Dynamic::new("value"))].into_boxed_slice().into()
}
```

Dynamic values can be arbitrarily nested, which opens up opportunities for rapid prototyping and elegant API design. On the other hand, static return types are more efficient, so you might prefer to use non-dynamic return types for the performance-sensitive parts of your API surface.