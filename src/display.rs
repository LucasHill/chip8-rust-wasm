#![feature(proc_macro)]

use DISPLAY_PIXEL_WIDTH;
use DISPLAY_PIXEL_HEIGHT;

const VRAM_SIZE: usize = DISPLAY_PIXEL_HEIGHT * DISPLAY_PIXEL_WIDTH;

pub struct Display {
  vram: [bool; VRAM_SIZE]
}

impl Display {
  pub fn new() -> Display {
    Display {
      vram: [false; VRAM_SIZE]
    }
  }

  pub fn clear(&mut self) {
    self.vram = [false; VRAM_SIZE]
  }

  fn set_pixel_to(&mut self, x: usize, y: usize, is_on: bool) {
    let scaled = y * DISPLAY_PIXEL_WIDTH;
    self.vram[x + scaled] = is_on;
  }

  fn is_pixel_on(&mut self, x: usize, y: usize) -> bool {
    let scaled = y * DISPLAY_PIXEL_WIDTH;
    self.vram[x + scaled] == true
  }

  pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
    //Todo: refactor
    let rows = sprite.len();
    let mut collision = false;
    for j in 0..rows {
      let row = sprite[j];
      for i in 0..8 {
        let new_value = row >> (7 - i) & 0x01;
        if new_value == 1 {
          let xi = (x + i) % DISPLAY_PIXEL_WIDTH;
          let yj = (y + j) % DISPLAY_PIXEL_HEIGHT;
          let old_value = self.is_pixel_on(xi, yj);
          if old_value {
            collision = true;
          }
          let is_on = (new_value == 1) ^ old_value;
          self.set_pixel_to(xi, yj, is_on);
        }
      }
    }
    return collision;
  }

  pub fn get_vram_copy(&self) -> Vec<u8> {
    self.vram.into_iter().map(|&x| if x { 1 } else { 0 }).collect()
  }
}