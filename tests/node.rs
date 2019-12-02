use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use wasm_feature_detect_sys as wasm;

#[wasm_bindgen_test]
async fn big_int() {
    let expected: JsValue = false.into();
    let observed: JsValue = JsFuture::from(wasm::big_int()).await.unwrap_throw();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
fn bulk_memory() {
    let expected = true;
    let observed = wasm::bulk_memory();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
fn exceptions() {
    let expected = false;
    let observed = wasm::exceptions();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
fn multi_value() {
    let expected = false;
    let observed = wasm::multi_value();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
fn mutable_globals() {
    let expected = true;
    let observed = wasm::mutable_globals();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
fn reference_types() {
    let expected = false;
    let observed = wasm::reference_types();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
fn saturated_float_to_int() {
    let expected = true;
    let observed = wasm::saturated_float_to_int();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
fn sign_extensions() {
    let expected = true;
    let observed = wasm::sign_extensions();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
fn simd() {
    let expected = false;
    let observed = wasm::simd();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
fn tail_call() {
    let expected = false;
    let observed = wasm::tail_call();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
fn threads() {
    let expected = false;
    let observed = wasm::threads();
    assert_eq!(expected, observed);
}
