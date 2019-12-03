//! Raw bindings to the wasm-feature-detect API for projects using wasm-bindgen

#![deny(clippy::all)]

use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "wasm-feature-detect")]
extern {
    /// Detector for the [BigInt Integration](https://github.com/WebAssembly/JS-BigInt-integration) feature proposal.
    #[wasm_bindgen(js_name = "bigInt")]
    pub fn big_int() -> Promise;

    /// Detector for the [Bulk Memory Operations](https://github.com/webassembly/bulk-memory-operations) feature proposal.
    #[wasm_bindgen(js_name = "bulkMemory")]
    pub fn bulk_memory() -> Promise;

    /// Detector for the [Exception Handling](https://github.com/WebAssembly/exception-handling) feature proposal.
    pub fn exceptions() -> Promise;

    /// Detector for the [Multi-Value](https://github.com/WebAssembly/multi-value) feature proposal.
    #[wasm_bindgen(js_name = "multiValue")]
    pub fn multi_value() -> Promise;

    /// Detector for the [Multi-Global](https://github.com/WebAssembly/mutable-global) feature proposal.
    #[wasm_bindgen(js_name = "mutableGlobals")]
    pub fn mutable_globals() -> Promise;

    /// Detector for the [Reference Types](https://github.com/WebAssembly/reference-types) feature proposal.
    #[wasm_bindgen(js_name = "referenceTypes")]
    pub fn reference_types() -> Promise;

    /// Detector for the [Non-Trapping Float-to-Int Conversion](https://github.com/WebAssembly/nontrapping-float-to-int-conversions) feature proposal.
    #[wasm_bindgen(js_name = "saturatedFloatToInt")]
    pub fn saturated_float_to_int() -> Promise;

    /// Detector for the [Sign-Extension Operators](https://github.com/WebAssembly/sign-extension-ops) feature proposal.
    #[wasm_bindgen(js_name = "signExtensions")]
    pub fn sign_extensions() -> Promise;

    /// Detector for the [SIMD](https://github.com/webassembly/simd) feature proposal.
    pub fn simd() -> Promise;

    /// Detector for the [Tail Call](https://github.com/webassembly/tail-call) feature proposal.
    #[wasm_bindgen(js_name = "tailCall")]
    pub fn tail_call() -> Promise;

    /// Detector for the [Threads](https://github.com/webassembly/threads) feature proposal.
    pub fn threads() -> Promise;
}
