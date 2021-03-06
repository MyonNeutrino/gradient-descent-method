//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

pub fn hello() {
    println!("Hello world!");
    println!("Hello world!");
}

#[wasm_bindgen_test]
fn pass() {
    hello();
    assert_eq!(1 + 1, 2);
}
