use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_func(){
    log::info!("It works!");
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}