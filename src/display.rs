#![feature(proc_macro)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use DISPLAY_PIXEL_WIDTH;
use DISPLAY_PIXEL_HEIGHT;

const VRAM_SIZE = DISPLAY_PIXEL_HEIGHT * DISPLAY_PIXEL_WIDTH;

#[wasm_bindgen]
pub struct Display {
  vram: [u8; VRAM_SIZE]
}

#[wasm_bindgen]
impl Display {
  pub fn new() -> Display {
    Display {
      vram: [0, VRAM_SIZE]
    }
  }
}