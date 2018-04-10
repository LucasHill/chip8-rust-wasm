#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

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

  pub fn get_memory(&self) -> Vec<u8> {
    self.memory.clone()
  }
}

#[test]
fn can_instantiate_cartridge() {
  let mut cartridge = Cartridge::new(&[2,3,4,5]);
  assert_eq!(cartridge.get_memory().len(), 4);
}
