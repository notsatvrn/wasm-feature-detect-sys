//! Raw bindings to the wasm-feature-detect API for projects using wasm-bindgen

#![deny(clippy::all)]

use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "wasm-feature-detect")]
extern {
    #[wasm_bindgen(js_name = "bigInt")]
    pub fn big_int() -> Promise;

    #[wasm_bindgen(js_name = "bulkMemory")]
    pub fn bulk_memory() -> Promise;

    pub fn exceptions() -> Promise;

    #[wasm_bindgen(js_name = "multiValue")]
    pub fn multi_value() -> Promise;

    #[wasm_bindgen(js_name = "mutableGlobals")]
    pub fn mutable_globals() -> Promise;

    #[wasm_bindgen(js_name = "referenceTypes")]
    pub fn reference_types() -> Promise;

    #[wasm_bindgen(js_name = "saturatedFloatToInt")]
    pub fn saturated_float_to_int() -> Promise;

    #[wasm_bindgen(js_name = "signExtensions")]
    pub fn sign_extensions() -> Promise;

    pub fn simd() -> Promise;

    #[wasm_bindgen(js_name = "tailCall")]
    pub fn tail_call() -> Promise;

    pub fn threads() -> Promise;
}
