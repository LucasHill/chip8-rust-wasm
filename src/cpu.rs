#![feature(proc_macro)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use cpu_helpers::ProgramCounterKind;
use cpu_helpers::OpcodeParts;
use cartridge::Cartridge;
use gamepad::Gamepad;
use font::FONT_SET;
use MEMORY_SIZE;

#[wasm_bindgen]
pub struct CPU {
  memory: [u8; MEMORY_SIZE],
  stack: [usize; 16],
  registers: [u8; 16],
  index: usize,
  program_counter: usize,
  delay_timer: u8,
  sound_timer: u8,
  gamepad: Gamepad
}

#[wasm_bindgen]
impl CPU {
  pub fn new(game: Cartridge) -> CPU {
    let mut memory = [0u8; MEMORY_SIZE];
    let game_memory = game.get_memory();

    memory[0..FONT_SET.len()].copy_from_slice(&FONT_SET);
    memory[0x200..0x200+game_memory.len()].copy_from_slice(&game_memory);

    CPU {
      memory,
      stack: [0; 16],
      registers: [0u8; 16],
      index: 0,
      program_counter: 0x200,
      delay_timer: 0,
      sound_timer: 0,
      gamepad: Gamepad::new()
    }
  }

  pub fn calculate_instruction(first_byte: u8, second_byte: u8) -> u16 {
   (first_byte as u16) << 8 | second_byte as u16
 }

  pub fn tick(&mut self) {
    let memory = self.memory;
    let pc = self.program_counter;
    let instruction = CPU::calculate_instruction(memory[pc], memory[pc + 1]);

    let parts = OpcodeParts::new(instruction);
    self.update_timers();
  }

  pub fn update_timers(&mut self) {
    if self.sound_timer > 0 {
      self.sound_timer -= 1;
    }

    if self.delay_timer > 0 {
      self.delay_timer -= 1;
    }
  }
}

#[test]
fn calculates_instruction_from_bytes() {
  let instruction = CPU::calculate_instruction(0xA2, 0xF0);
  assert_eq!(instruction, 0xA2F0);
}

#[test]
fn decrements_timers() {
  let mut cpu = CPU::new(Cartridge::new(&[2,3,4,5]));
  cpu.sound_timer = 4;
  cpu.tick();

  assert_eq!(cpu.sound_timer, 3);
}

#[test]
fn does_not_decrement_timers_at_0() {
  let mut cpu = CPU::new(Cartridge::new(&[2,3,4,5]));
  cpu.tick();

  assert_eq!(cpu.sound_timer, 0);
  assert_eq!(cpu.delay_timer, 0);
}