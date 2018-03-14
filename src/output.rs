#![feature(proc_macro)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct OutputState {
  pub test: u8
}

#[wasm_bindgen]
impl OutputState {
  pub fn new(test: u8) -> OutputState {
    OutputState {
      test: test
    }
  }

  pub fn get(&self) -> u8 {
    self.test
  }
}
