mod cube;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Universe;

#[wasm_bindgen]
pub fn init() -> Universe {
    utils::set_panic_hook();
    Universe
}

#[wasm_bindgen]
pub fn greet() {
    // alert("Hello, autocuber!");
    let cube = cube::Cube::<3>::new();
    utils::log!("cube:\n{}", cube);
}
