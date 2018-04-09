#![feature(proc_macro)]

extern crate wasm_bindgen;
extern crate rand;

const MEMORY_SIZE: usize = 4096;
const DISPLAY_PIXEL_WIDTH: usize = 64;
const DISPLAY_PIXEL_HEIGHT: usize = 32;

pub mod cpu;
pub mod cartridge;
pub mod output;
mod font;
mod gamepad;
mod display;
mod cpu_helpers;