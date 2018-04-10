use MEMORY_SIZE;
use wasm_bindgen::prelude::*;

pub enum ProgramCounterKind {
  Next,
  Skip,
  Jump(usize)
}

impl ProgramCounterKind {
  pub fn skip_if(skip: bool) -> ProgramCounterKind {
    return if skip { ProgramCounterKind::Skip } else { ProgramCounterKind::Next }
  }
}

pub struct OpcodeParts {
  nibbles: (u8, u8, u8, u8),
  pub x: usize,
  pub y: usize,
  pub n: usize,
  pub nnn: usize,
  pub kk: u8
}

impl OpcodeParts {
  pub fn new(instruction: u16) -> OpcodeParts {

    let nibbles = (
      ((instruction & 0xF000) >> 12) as u8, 
      ((instruction & 0x0F00) >> 8) as u8, 
      ((instruction & 0x00F0) >> 4) as u8, 
      (instruction & 0x000F) as u8
    );

    let x = nibbles.1 as usize;
    let y = nibbles.2 as usize;
    let n = nibbles.3 as usize;
    let nnn = (instruction & 0x0FFF) as usize;
    let kk = (instruction & 0x00FF) as u8;

    OpcodeParts { nibbles, x , y, n, nnn, kk }
  }

  pub fn get_nibbles(&self) -> (u8, u8, u8, u8) {
    self.nibbles
  }
}

#[wasm_bindgen]
pub struct ExecutionResult {
  pub display_state: Vec<u8>,
  pub should_beep: bool
}

#[wasm_bindgen]
impl ExecutionResult {
  pub fn new(display_state: Vec<u8>, should_beep: bool) -> ExecutionResult {
    ExecutionResult {
      display_state,
      should_beep
    }
  }
}

#[test]
fn program_counter_skips_if_true() {
  let result = ProgramCounterKind::skip_if(true);
  match result {
    ProgramCounterKind::Skip => assert!(true),
    _ => assert!(false)
  }
}

#[test]
fn program_counter_doesnt_skip_if_false() {
  let result = ProgramCounterKind::skip_if(false);
  match result {
    ProgramCounterKind::Next => assert!(true),
    _ => assert!(false)
  }
}

