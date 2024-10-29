use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}

