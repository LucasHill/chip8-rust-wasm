#![feature(proc_macro)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use DISPLAY_PIXEL_WIDTH;
use DISPLAY_PIXEL_HEIGHT;

const VRAM_SIZE: usize = DISPLAY_PIXEL_HEIGHT * DISPLAY_PIXEL_WIDTH;

#[wasm_bindgen]
pub struct Display {
  vram: [bool; VRAM_SIZE]
}

#[wasm_bindgen]
impl Display {
  pub fn new() -> Display {
    Display {
      vram: [false; VRAM_SIZE]
    }
  }

  pub fn clear(&mut self) {
    self.vram = [false; VRAM_SIZE]
  }

  pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
    //todo implement
    true
  }
}