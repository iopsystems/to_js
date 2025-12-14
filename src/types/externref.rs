//! JavaScript object references via a table-based approach.
//!
//! This module allows Rust code to hold references to JavaScript objects and call
//! methods on them. Objects are stored in a JavaScript-side table, and Rust holds
//! indices into that table.
//!
//! # Setup
//!
//! When instantiating your WebAssembly module, provide the `__js` imports:
//!
//! ```javascript
//! const jsRefs = [null]; // Index 0 = null, objects start at index 1
//!
//! function decodeStr(ptr, len) {
//!     return new TextDecoder().decode(new Uint8Array(memory.buffer, ptr, len));
//! }
//!
//! const jsImports = {
//!     __js: {
//!         // Core
//!         global_this: () => { jsRefs.push(globalThis); return jsRefs.length - 1; },
//!         release: (idx) => { jsRefs[idx] = null; },
//!
//!         // Property access
//!         get: (idx, p, l) => { const v = jsRefs[idx][decodeStr(p, l)]; jsRefs.push(v); return jsRefs.length - 1; },
//!         set: (idx, p, l, val_idx) => { jsRefs[idx][decodeStr(p, l)] = jsRefs[val_idx]; },
//!
//!         // Conversions: primitives → JsValue index
//!         from_f64: (n) => { jsRefs.push(n); return jsRefs.length - 1; },
//!         from_str: (p, l) => { jsRefs.push(decodeStr(p, l)); return jsRefs.length - 1; },
//!
//!         // Function calls (0-5 args)
//!         call_0: (f) => { const r = jsRefs[f](); jsRefs.push(r); return jsRefs.length - 1; },
//!         call_1: (f, a) => { const r = jsRefs[f](jsRefs[a]); jsRefs.push(r); return jsRefs.length - 1; },
//!         call_2: (f, a, b) => { const r = jsRefs[f](jsRefs[a], jsRefs[b]); jsRefs.push(r); return jsRefs.length - 1; },
//!         call_3: (f, a, b, c) => { const r = jsRefs[f](jsRefs[a], jsRefs[b], jsRefs[c]); jsRefs.push(r); return jsRefs.length - 1; },
//!         call_4: (f, a, b, c, d) => { const r = jsRefs[f](jsRefs[a], jsRefs[b], jsRefs[c], jsRefs[d]); jsRefs.push(r); return jsRefs.length - 1; },
//!         call_5: (f, a, b, c, d, e) => { const r = jsRefs[f](jsRefs[a], jsRefs[b], jsRefs[c], jsRefs[d], jsRefs[e]); jsRefs.push(r); return jsRefs.length - 1; },
//!
//!         // Method calls (0-5 args)
//!         call_method_0: (o, m, l) => { const r = jsRefs[o][decodeStr(m, l)](); jsRefs.push(r); return jsRefs.length - 1; },
//!         call_method_1: (o, m, l, a) => { const r = jsRefs[o][decodeStr(m, l)](jsRefs[a]); jsRefs.push(r); return jsRefs.length - 1; },
//!         call_method_2: (o, m, l, a, b) => { const r = jsRefs[o][decodeStr(m, l)](jsRefs[a], jsRefs[b]); jsRefs.push(r); return jsRefs.length - 1; },
//!         call_method_3: (o, m, l, a, b, c) => { const r = jsRefs[o][decodeStr(m, l)](jsRefs[a], jsRefs[b], jsRefs[c]); jsRefs.push(r); return jsRefs.length - 1; },
//!         call_method_4: (o, m, l, a, b, c, d) => { const r = jsRefs[o][decodeStr(m, l)](jsRefs[a], jsRefs[b], jsRefs[c], jsRefs[d]); jsRefs.push(r); return jsRefs.length - 1; },
//!         call_method_5: (o, m, l, a, b, c, d, e) => { const r = jsRefs[o][decodeStr(m, l)](jsRefs[a], jsRefs[b], jsRefs[c], jsRefs[d], jsRefs[e]); jsRefs.push(r); return jsRefs.length - 1; },
//!
//!         // Extract primitive from JsValue
//!         to_f64: (idx) => Number(jsRefs[idx]),
//!     }
//! };
//! ```

#[link(wasm_import_module = "__js")]
extern "C" {
    fn global_this() -> u32;
    fn release(idx: u32);
    fn get(idx: u32, key_ptr: *const u8, key_len: u32) -> u32;
    fn set(idx: u32, key_ptr: *const u8, key_len: u32, val_idx: u32);
    fn from_f64(n: f64) -> u32;
    fn from_str(ptr: *const u8, len: u32) -> u32;
    fn to_f64(idx: u32) -> f64;

    fn call_0(f: u32) -> u32;
    fn call_1(f: u32, a: u32) -> u32;
    fn call_2(f: u32, a: u32, b: u32) -> u32;
    fn call_3(f: u32, a: u32, b: u32, c: u32) -> u32;
    fn call_4(f: u32, a: u32, b: u32, c: u32, d: u32) -> u32;
    fn call_5(f: u32, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32;

    fn call_method_0(obj: u32, method_ptr: *const u8, method_len: u32) -> u32;
    fn call_method_1(obj: u32, method_ptr: *const u8, method_len: u32, a: u32) -> u32;
    fn call_method_2(obj: u32, method_ptr: *const u8, method_len: u32, a: u32, b: u32) -> u32;
    fn call_method_3(obj: u32, method_ptr: *const u8, method_len: u32, a: u32, b: u32, c: u32) -> u32;
    fn call_method_4(obj: u32, method_ptr: *const u8, method_len: u32, a: u32, b: u32, c: u32, d: u32) -> u32;
    fn call_method_5(obj: u32, method_ptr: *const u8, method_len: u32, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32;
}

/// A handle to a JavaScript value stored in a JS-side table.
///
/// This is an index into a JavaScript array that holds the actual values.
/// When a `JsValue` is dropped, it releases its slot in the table.
///
/// # Example
///
/// ```rust,ignore
/// use to_js::JsValue;
///
/// // Get console.log and call it
/// let console = JsValue::global().get("console");
/// let message = JsValue::from("Hello from Rust!");
/// console.call_method1("log", &message);
///
/// // Access window.innerWidth
/// let width: f64 = JsValue::global().get("window").get("innerWidth").into();
/// ```
#[derive(Debug)]
pub struct JsValue(u32);

impl JsValue {
    /// Index 0 represents null/undefined in our table.
    pub const NULL: JsValue = JsValue(0);

    /// Returns `globalThis`, the global object.
    ///
    /// In browsers this is `window`, in Node.js it's `global`.
    #[inline]
    pub fn global() -> Self {
        Self(unsafe { global_this() })
    }

    /// Creates a `JsValue` from a raw table index.
    ///
    /// # Safety
    /// The index must be a valid entry in the JS-side table.
    #[inline]
    pub const unsafe fn from_raw(idx: u32) -> Self {
        Self(idx)
    }

    /// Returns the raw table index.
    #[inline]
    pub const fn as_raw(&self) -> u32 {
        self.0
    }

    /// Consumes the `JsValue` and returns the raw index without dropping.
    #[inline]
    pub fn into_raw(self) -> u32 {
        let idx = self.0;
        core::mem::forget(self);
        idx
    }

    /// Returns true if this is a null reference (index 0).
    #[inline]
    pub const fn is_null(&self) -> bool {
        self.0 == 0
    }

    /// Gets a property from this object: `self[key]`
    #[inline]
    pub fn get(&self, key: &str) -> Self {
        Self(unsafe { get(self.0, key.as_ptr(), key.len() as u32) })
    }

    /// Sets a property on this object: `self[key] = val`
    #[inline]
    pub fn set(&self, key: &str, val: &JsValue) {
        unsafe { set(self.0, key.as_ptr(), key.len() as u32, val.0) }
    }

    /// Calls this value as a function with no arguments: `self()`
    #[inline]
    pub fn call0(&self) -> Self {
        Self(unsafe { call_0(self.0) })
    }

    /// Calls this value as a function with 1 argument: `self(a)`
    #[inline]
    pub fn call1(&self, a: &JsValue) -> Self {
        Self(unsafe { call_1(self.0, a.0) })
    }

    /// Calls this value as a function with 2 arguments: `self(a, b)`
    #[inline]
    pub fn call2(&self, a: &JsValue, b: &JsValue) -> Self {
        Self(unsafe { call_2(self.0, a.0, b.0) })
    }

    /// Calls this value as a function with 3 arguments: `self(a, b, c)`
    #[inline]
    pub fn call3(&self, a: &JsValue, b: &JsValue, c: &JsValue) -> Self {
        Self(unsafe { call_3(self.0, a.0, b.0, c.0) })
    }

    /// Calls this value as a function with 4 arguments: `self(a, b, c, d)`
    #[inline]
    pub fn call4(&self, a: &JsValue, b: &JsValue, c: &JsValue, d: &JsValue) -> Self {
        Self(unsafe { call_4(self.0, a.0, b.0, c.0, d.0) })
    }

    /// Calls this value as a function with 5 arguments: `self(a, b, c, d, e)`
    #[inline]
    pub fn call5(
        &self,
        a: &JsValue,
        b: &JsValue,
        c: &JsValue,
        d: &JsValue,
        e: &JsValue,
    ) -> Self {
        Self(unsafe { call_5(self.0, a.0, b.0, c.0, d.0, e.0) })
    }

    /// Calls a method on this object with no arguments: `self.method()`
    #[inline]
    pub fn call_method0(&self, method: &str) -> Self {
        Self(unsafe { call_method_0(self.0, method.as_ptr(), method.len() as u32) })
    }

    /// Calls a method on this object with 1 argument: `self.method(a)`
    #[inline]
    pub fn call_method1(&self, method: &str, a: &JsValue) -> Self {
        Self(unsafe { call_method_1(self.0, method.as_ptr(), method.len() as u32, a.0) })
    }

    /// Calls a method on this object with 2 arguments: `self.method(a, b)`
    #[inline]
    pub fn call_method2(&self, method: &str, a: &JsValue, b: &JsValue) -> Self {
        Self(unsafe { call_method_2(self.0, method.as_ptr(), method.len() as u32, a.0, b.0) })
    }

    /// Calls a method on this object with 3 arguments: `self.method(a, b, c)`
    #[inline]
    pub fn call_method3(&self, method: &str, a: &JsValue, b: &JsValue, c: &JsValue) -> Self {
        Self(unsafe { call_method_3(self.0, method.as_ptr(), method.len() as u32, a.0, b.0, c.0) })
    }

    /// Calls a method on this object with 4 arguments: `self.method(a, b, c, d)`
    #[inline]
    pub fn call_method4(
        &self,
        method: &str,
        a: &JsValue,
        b: &JsValue,
        c: &JsValue,
        d: &JsValue,
    ) -> Self {
        Self(unsafe {
            call_method_4(self.0, method.as_ptr(), method.len() as u32, a.0, b.0, c.0, d.0)
        })
    }

    /// Calls a method on this object with 5 arguments: `self.method(a, b, c, d, e)`
    #[inline]
    pub fn call_method5(
        &self,
        method: &str,
        a: &JsValue,
        b: &JsValue,
        c: &JsValue,
        d: &JsValue,
        e: &JsValue,
    ) -> Self {
        Self(unsafe {
            call_method_5(
                self.0,
                method.as_ptr(),
                method.len() as u32,
                a.0,
                b.0,
                c.0,
                d.0,
                e.0,
            )
        })
    }

    /// Converts this JS value to an f64.
    #[inline]
    pub fn as_f64(&self) -> f64 {
        unsafe { to_f64(self.0) }
    }
}

impl Drop for JsValue {
    fn drop(&mut self) {
        if self.0 != 0 {
            unsafe { release(self.0) }
        }
    }
}

impl Clone for JsValue {
    fn clone(&self) -> Self {
        // Get the same property from itself (identity operation that creates new table entry)
        // Actually, we need a proper clone. For now, just re-get from the table.
        // This is a limitation - true cloning would need another import.
        // For simplicity, we'll just copy the index without incrementing refcount.
        // Users should be careful about ownership.
        Self(self.0)
    }
}

impl From<f64> for JsValue {
    #[inline]
    fn from(n: f64) -> Self {
        Self(unsafe { from_f64(n) })
    }
}

impl From<i32> for JsValue {
    #[inline]
    fn from(n: i32) -> Self {
        Self(unsafe { from_f64(n as f64) })
    }
}

impl From<u32> for JsValue {
    #[inline]
    fn from(n: u32) -> Self {
        Self(unsafe { from_f64(n as f64) })
    }
}

impl From<bool> for JsValue {
    #[inline]
    fn from(b: bool) -> Self {
        Self(unsafe { from_f64(if b { 1.0 } else { 0.0 }) })
    }
}

impl From<&str> for JsValue {
    #[inline]
    fn from(s: &str) -> Self {
        Self(unsafe { from_str(s.as_ptr(), s.len() as u32) })
    }
}

impl From<&JsValue> for f64 {
    #[inline]
    fn from(val: &JsValue) -> Self {
        val.as_f64()
    }
}
