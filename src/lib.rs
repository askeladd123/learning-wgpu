mod window;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn run(){

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut data = window::Data::new();
    window::run(&mut data);
}