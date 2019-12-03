use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use wasm_feature_detect_sys as wasm;

#[wasm_bindgen_test]
async fn big_int() {
    let expected: JsValue = false.into();
    let observed = JsFuture::from(wasm::big_int()).await.unwrap_throw();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
async fn bulk_memory() {
    let expected: JsValue = true.into();
    let observed = JsFuture::from(wasm::bulk_memory()).await.unwrap_throw();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
async fn exceptions() {
    let expected: JsValue = false.into();
    let observed = JsFuture::from(wasm::exceptions()).await.unwrap_throw();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
async fn multi_value() {
    let expected: JsValue = false.into();
    let observed = JsFuture::from(wasm::multi_value()).await.unwrap_throw();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
async fn mutable_globals() {
    let expected: JsValue = true.into();
    let observed = JsFuture::from(wasm::mutable_globals()).await.unwrap_throw();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
async fn reference_types() {
    let expected: JsValue = false.into();
    let observed = JsFuture::from(wasm::reference_types()).await.unwrap_throw();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
async fn saturated_float_to_int() {
    let expected: JsValue = true.into();
    let observed = JsFuture::from(wasm::saturated_float_to_int()).await.unwrap_throw();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
async fn sign_extensions() {
    let expected: JsValue = true.into();
    let observed = JsFuture::from(wasm::sign_extensions()).await.unwrap_throw();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
async fn simd() {
    let expected: JsValue = false.into();
    let observed = JsFuture::from(wasm::simd()).await.unwrap_throw();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
async fn tail_call() {
    let expected: JsValue = false.into();
    let observed = JsFuture::from(wasm::tail_call()).await.unwrap_throw();
    assert_eq!(expected, observed);
}

#[wasm_bindgen_test]
async fn threads() {
    let expected: JsValue = false.into();
    let observed = JsFuture::from(wasm::threads()).await.unwrap_throw();
    assert_eq!(expected, observed);
}
