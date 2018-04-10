use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_usize(a: usize);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u16(a: u16);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u8(a: u8);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);
    #[wasm_bindgen(js_namespace = window, js_name = generateRandomU8)]
    pub fn generate_random_u8() -> u8;
}
