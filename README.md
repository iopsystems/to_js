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

## Usage

```js
// Given a WebAssembly instance, return an object containing its #[js] exports.
// If the optional second argument is true, typed arrays (including ones that
// were stashed or returned as packed arrays) will be copied out of WebAssembly
// memory before being returned, enhancing ease-of-use at the cost of extra data copies.
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

## Memory management

Returning owned values is accomplished by wrapping them in `KeepAlive`, which ensures the value lives until the next FFI call from JS to a Rust function.

```rust
use to_js::{KeepAlive};

#[js]
fn string() -> KeepAlive<String> {
    KeepAlive::new("Hello from a String".to_string())
}

#[js]
fn count_vec(count_up_to: usize) -> KeepAlive<Vec<usize>> {
    KeepAlive::new((1..=count_up_to).collect::<Vec<_>>())
}

#[js]
fn count_vec_result(count_up_to: usize) -> Result<KeepAlive<Vec<usize>>, &'static str> {
    if count_up_to > 100 {
        return Err("I can't count that high.");
    }
    Ok(count_vec(count_up_to))
}
```

Alternatively, to hand the responsibility for lifetime management over to JavaScript, use the provided functions `alloc` and `dealloc`.

<details>
    <summary>Here's a real-world example that defines an <a href='https://h2histogram.org'>H2 histogram</a> type whose lifetime is managed by JavaScript.</summary>

```rust
use to_js::{alloc, dealloc};

#[derive(Copy, Clone)]
struct H2 {
    a: u32,
    b: u32,
}

impl H2 {
    fn new(a: u32, b: u32) -> Self {
        H2 { a, b }
    }

    fn encode(self, value: u32) -> u32 {
        let H2 { a, b } = self;
        let c = a + b + 1;
        if value < (1 << c) {
            value >> a
        } else {
            let log_segment = 31 - value.leading_zeros();
            (value >> (log_segment - b)) + ((log_segment - c + 1) << b)
        }
    }

    fn decode(self, code: u32) -> [u32; 2] {
        let H2 { a, b } = self;
        let c = a + b + 1;
        let bins_below_cutoff = 1 << (c - a);
        let lower: u32;
        let bin_width: u32;
        if code < bins_below_cutoff {
            // we're in the linear section of the histogram where each bin is 2^a wide
            lower = code << a;
            bin_width = 1 << a;
        } else {
            // we're in the log section of the histogram with 2^b bins per log segment
            let log_segment = c + ((code - bins_below_cutoff) >> b);
            let bin_offset = code & ((1 << b) - 1);
            lower = (1 << log_segment) + (bin_offset << (log_segment - b));
            bin_width = 1 << (log_segment - b);
        };
        [lower, lower + (bin_width - 1)]
    }
}

#[js]
fn h2_alloc(a: u32, b: u32) -> Result<*mut H2, &'static str> {
    if a + b + 1 > 31 {
        return Err("a + b + 1 must be < 32 or operations will overflow");
    }
    Ok(alloc(H2::new(a, b)))
}

#[js]
fn h2_encode(x: &H2, value: u32) -> u32 {
    x.encode(value)
}

#[js]
fn h2_decode(x: &H2, code: u32) -> U32Pair {
    U32Pair(x.decode(code))
}

#[js]
fn h2_dealloc(ptr: *mut H2) {
    dealloc(ptr);
}
```

On the JavaScript side you can use the following helper function to construct a JavaScript constructor function that uses these methods.

```js
// Convenience method to generate a JavaScript-side class that corresponds to a Rust-side struct.
// Convenience method to generate a JavaScript-side class that corresponds to a Rust-side struct.
function createClass(
  // The WebAssembly instance wrapper returned by `toJs(instance)`
  instance,
  // Name prefix shared by all methods, without a trailing underscore
  prefix,
  {
    // Array of method names
    methods,
    // Optional constructor function to override the default of `instance[prefix + 'alloc']`
    alloc,
    // Optional object from method name to wrapper function that can transform the return value of the method.
    transforms,
  },
) {
  if (prefix.endsWith("_"))
    throw new Error(
      "name prefix should not include a trailing underscore, as one will be added automatically",
    );
  prefix += "_";

  // Ensure that "dealloc" is a method on the class
  if (!methods.includes("dealloc")) methods.push("dealloc");

  // Create the constructor function and add method definitions to its prototype
  const Class = function (...args) {
    this.ptr = (alloc ?? instance[prefix + "alloc"])(...args);
  };

  const identity = (x) => x;

  for (const name of methods) {
    const method = instance[prefix + name];
    if (method === undefined)
      throw new Error("undefined method: " + (prefix + name));
    const transform = transforms?.[name] ?? identity;
    Class.prototype[name] = function (...args) {
      return transform(method(this.ptr, ...args));
    };
  }
  return Class
}
```

This function can be used to define `H2` and use it:

```js
const H2 = createClass(rs, "h2", { names: ["encode", "decode"] })

const hist = new H2(1, 8);      // Construct a Rust-side H2 histogram struct
const value = hist.encode(123); // Use it
hist.dealloc();                 // Deallocate it when finished
```

</details>

## Packed arrays

This library encodes all returned values into 64 bits with type information passed through a side channel. A nice consequence is that we can efficiently return small fixed-size ("packed") arrays without extra allocation, so long as they fit into 64 bits. 

Packed arrays will be returned as the appropriate type of typed array, reusing the same typed array object between calls. Note that all packed array data is internally represented by the same single-element `Float64Array`, so calls to any function returning a packed array will invalidate the results from the previous call that returned a packed array.

The packed array types are `U8Octet`, `I8Octet`, `U16Quartet`, `I16Quartet`, `U32Pair`, `I32Pair`, and `F32Pair`.

```rs
use to_js::{U8Octet, I8Octet, U16Quartet, F32Pair};

#[js]
fn octet() -> U8Octet {
    U8Octet([1, 2, 3, 4, 5, 6, 7, 8])
}

#[js]
fn i8_octet() -> I8Octet {
    I8Octet([-1, -2, -3, -4, 5, 6, 7, 8])
}

#[js]
fn u16_quartet() -> U16Quartet {
    U16Quartet([1, 2, 3, 4])
}

#[js]
fn f32_pair() -> F32Pair {
    F32Pair([1.0, 2.0])
}
```

## Dynamic return types

As an experimental feature you can return dynamically-typed values using the `Dynamic` type:

```rust
use to_js::Dynamic;

#[js]
fn string_or_int(x: u32) -> Result<Dynamic, &'static str> {
    match x {
        0..10 => Ok(Dynamic::new("hi from a String".to_string())),
        10..100 => Ok(Dynamic::new(123)),
        _ => Err("no dynamic for you!"),
    }
}
```

You can can return dynamic arrays, which are represented on the other side of the FFI boundary as plain [JavaScript arrays](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array):

```rust
#[js]
fn dynamic_array() -> KeepAlive<Box<[Dynamic]>> {
    KeepAlive::new(
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

`Dynamic` values can be arbitrarily nested, which opens up opportunities for rapid prototyping and elegant API design. On the other hand, static return types are more efficient, so you might prefer to use non-dynamic return types for the performance-sensitive parts of your API surface.

## Serializing as JSON

Another option for flexible high-level data exchange is to serialize your data as JSON, which can be done by enabling the `json` crate feature. This allows you to use [serde](https://github.com/serde-rs/serde) to encode arbitrary types that are serialized as JSON across the language boundary.

You can enable the feature by adding `features = ["json"]` to your `to_js` dependency in `Cargo.toml`.

```rust
use serde::Serialize;
use to_js::{Json};

#[derive(Serialize)]
struct TestStruct {
    x: u32,
    y: String,
}

#[js]
fn test_json() -> Json {
    Json::new(TestStruct {
        x: 123,
        y: "456!".to_string(),
    })
}
```

Calling this function from JavaScript will return a JavaScript object: `{ x: 123, y: "456!" }`.
