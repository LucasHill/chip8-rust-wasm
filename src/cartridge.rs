#![feature(proc_macro)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use output::OutputState;

#[wasm_bindgen]
pub struct Cartridge {
  memory: Vec<u8>
}

#[wasm_bindgen]
impl Cartridge {
  pub fn new(data: &[u8]) -> Cartridge {

    Cartridge {
      memory: data.to_vec()
    }
  }

  pub fn get_output(&self) -> OutputState {
    OutputState { test: self.memory[0] }
  }
}
