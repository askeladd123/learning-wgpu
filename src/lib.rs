use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run(){
    web_sys::console::log_1(&"hello man".into());
}