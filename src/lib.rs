#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

const MEMORY_SIZE: usize = 4096;
const DISPLAY_PIXEL_WIDTH: usize = 64;
const DISPLAY_PIXEL_HEIGHT: usize = 32;

pub mod cpu;
pub mod cartridge;
pub mod font;
pub mod gamepad;
pub mod display;
pub mod cpu_helpers;
pub mod js_interop;
