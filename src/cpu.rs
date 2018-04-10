
use wasm_bindgen::prelude::*;

use cpu_helpers::ProgramCounterKind;
use cpu_helpers::OpcodeParts;
use cpu_helpers::ExecutionResult;
use cartridge::Cartridge;
use gamepad::Gamepad;
use display::Display;
use font::FONT_SET;

use js_interop::log_u16;
use js_interop::log_u8;
use js_interop::log;
use js_interop::log_usize;
use js_interop::generate_random_u8;
use MEMORY_SIZE;

#[wasm_bindgen]
pub struct CPU {
  memory: [u8; MEMORY_SIZE],
  display: Display,
  stack: [usize; 16],
  stack_pointer: usize,
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
      stack_pointer: 0,
      display: Display::new(),
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

  pub fn gamepad_down(&mut self, key: &str) {
    self.gamepad.set_key_down(key)
  }

  pub fn gamepad_up(&mut self, key: &str) {
    self.gamepad.set_key_up(key)
  }

  pub fn tick(&mut self) -> ExecutionResult {
    let memory = self.memory;
    let pc = self.program_counter;
    let instruction = CPU::calculate_instruction(memory[pc], memory[pc + 1]);

    log_u16(instruction);

    let parts = OpcodeParts::new(instruction);
    self.update_timers();
    self.execute_instruction(&parts);

    ExecutionResult::new(self.display.get_vram_copy(), self.sound_timer > 0)
  }

  fn update_timers(&mut self) {
    if self.sound_timer > 0 {
      self.sound_timer -= 1;
    }

    if self.delay_timer > 0 {
      self.delay_timer -= 1;
    }
  }

  fn execute_instruction(&mut self, parts: &OpcodeParts) {
    let next_program_count = match parts.get_nibbles() {
      (0, 0, 0xE, 0x0) => self.run_00E0(),
      (0, 0, 0xE, 0xE) => self.run_00EE(),
      (0x1, _, _, _) => self.run_1NNN(parts.nnn),
      (0x2, _, _, _) => self.run_2NNN(parts.nnn),
      (0x3, _, _, _) => self.run_3xkk(parts.x, parts.kk),
      (0x4, _, _, _) => self.run_4xkk(parts.x, parts.kk),
      (0x5, _, _, _) => self.run_5xy0(parts.x, parts.y),
      (0x6, _, _, _) => self.run_6xkk(parts.x, parts.kk),
      (0x7, _, _, _) => self.run_7xkk(parts.x, parts.kk),
      (0x8, _, _, 0x0) => self.run_8xy0(parts.x, parts.y),
      (0x8, _, _, 0x1) => self.run_8xy1(parts.x, parts.y),
      (0x8, _, _, 0x2) => self.run_8xy2(parts.x, parts.y),
      (0x8, _, _, 0x3) => self.run_8xy3(parts.x, parts.y),
      (0x8, _, _, 0x4) => self.run_8xy4(parts.x, parts.y),
      (0x8, _, _, 0x5) => self.run_8xy5(parts.x, parts.y),
      (0x8, _, _, 0x6) => self.run_8xy6(parts.x),
      (0x8, _, _, 0x7) => self.run_8xy7(parts.x, parts.y),
      (0x8, _, _, 0xE) => self.run_8xyE(parts.x),
      (0x9, _, _, _) => self.run_9xy0(parts.x, parts.y),
      (0xA, _, _, _) => self.run_ANNN(parts.nnn),
      (0xB, _, _, _) => self.run_BNNN(parts.nnn),
      (0xC, _, _, _) => self.run_Cxkk(parts.x, parts.kk),
      (0xD, _, _, _) => self.run_Dxyn(parts.x, parts.y, parts.n),
      (0xE, _, _, 0xE) => self.run_Ex9E(parts.x),
      (0xE, _, _, 0x1) => self.run_ExA1(parts.x),
      (0xF, _, 0x0, 0x7) => self.run_Fx07(parts.x),
      (0xF, _, 0x0, 0xA) => self.run_Fx0A(parts.x),
      (0xF, _, 0x1, 0x5) => self.run_Fx15(parts.x),
      (0xF, _, 0x1, 0x8) => self.run_Fx18(parts.x),
      (0xF, _, 0x1, 0xE) => self.run_Fx1E(parts.x),
      (0xF, _, 0x2, 0x9) => self.run_Fx29(parts.x),
      (0xF, _, 0x3, 0x3) => self.run_Fx33(parts.x),
      (0xF, _, 0x5, 0x5) => self.run_Fx55(parts.x),
      (0xF, _, 0x6, 0x5) => self.run_Fx65(parts.x),

      _ => ProgramCounterKind::Next
    };

    match next_program_count {
      ProgramCounterKind::Next => self.program_counter += 2,
      ProgramCounterKind::Skip => self.program_counter += 4,
      ProgramCounterKind::Jump(n) => self.program_counter = n
    }
  }

  fn run_00E0(&mut self) -> ProgramCounterKind {
    self.display.clear();
    ProgramCounterKind::Next
  } 

  fn run_00EE(&mut self) -> ProgramCounterKind {
    self.stack_pointer -= 1;

    ProgramCounterKind::Jump(self.stack[self.stack_pointer])
  } 

  fn run_1NNN(&mut self, nnn: usize) -> ProgramCounterKind {
    ProgramCounterKind::Jump(nnn)
  }

  fn run_2NNN(&mut self, nnn: usize) -> ProgramCounterKind {
    self.stack[self.stack_pointer] = self.program_counter + 2;
    self.stack_pointer += 1;
    log("nnn: ");
    log_usize(nnn);
    ProgramCounterKind::Jump(nnn)
  }

  fn run_3xkk(&mut self, x: usize, kk: u8) -> ProgramCounterKind {
    ProgramCounterKind::skip_if(self.registers[x] == kk)
  }

  fn run_4xkk(&mut self, x: usize, kk: u8) -> ProgramCounterKind {
    ProgramCounterKind::skip_if(self.registers[x] != kk)
  }

  fn run_5xy0(&mut self, x: usize, y: usize) -> ProgramCounterKind {
    ProgramCounterKind::skip_if(self.registers[x] == self.registers[y])
  }

  fn run_6xkk(&mut self, x: usize, kk: u8) -> ProgramCounterKind {
    self.registers[x] = kk;
    ProgramCounterKind::Next
  }

  fn run_7xkk(&mut self, x: usize, kk: u8) -> ProgramCounterKind {
    self.registers[x] += kk;
    ProgramCounterKind::Next
  }

  fn run_8xy0(&mut self, x: usize, y: usize) -> ProgramCounterKind {
    self.registers[x] = self.registers[y];
    ProgramCounterKind::Next
  }

  fn run_8xy1(&mut self, x: usize, y: usize) -> ProgramCounterKind {
    self.registers[x] = self.registers[x] | self.registers[y];
    ProgramCounterKind::Next
  }

  fn run_8xy2(&mut self, x: usize, y: usize) -> ProgramCounterKind {
    self.registers[x] = self.registers[x] & self.registers[y];
    ProgramCounterKind::Next
  }

  fn run_8xy3(&mut self, x: usize, y: usize) -> ProgramCounterKind {
    self.registers[x] = self.registers[x] ^ self.registers[y];
    ProgramCounterKind::Next
  }

  fn run_8xy4(&mut self, x: usize, y: usize) -> ProgramCounterKind {
    let total =  self.registers[x] as u16 + self.registers[y] as u16;
    self.registers[0xF] = if total > 0xFF { 1 } else { 0 };
    self.registers[x] = total as u8;
    ProgramCounterKind::Next
  }

  fn run_8xy5(&mut self, x: usize, y: usize) -> ProgramCounterKind {
    let vx = self.registers[x];
    let vy = self.registers[y];

    self.registers[0xF] = if vx > vy { 1 } else { 0 };
    self.registers[x] = vx.wrapping_sub(vy);

    ProgramCounterKind::Next
  }

  fn run_8xy6(&mut self, x: usize) -> ProgramCounterKind {
    self.registers[0xF] = self.registers[x] & 0x1;
    self.registers[x] >>= 1;

    ProgramCounterKind::Next
  }

  fn run_8xy7(&mut self, x: usize, y: usize) -> ProgramCounterKind {
    let vx = self.registers[x];
    let vy = self.registers[y];

    self.registers[0xF] = if vy > vx { 1 } else { 0 };
    self.registers[x] = vy.wrapping_sub(vx);

    ProgramCounterKind::Next
  }

  fn run_8xyE(&mut self, x: usize) -> ProgramCounterKind {
    self.registers[0xF] = (self.registers[x] & 0x80) >> 7;
    self.registers[x] <<= 1;

    ProgramCounterKind::Next
  }

  fn run_9xy0(&mut self, x: usize, y: usize) -> ProgramCounterKind {
    ProgramCounterKind::skip_if(self.registers[x] != self.registers[y])
  }

  fn run_ANNN(&mut self, nnn: usize) -> ProgramCounterKind {
    self.index = nnn;
    ProgramCounterKind::Next
  }

  fn run_BNNN(&mut self, nnn: usize) -> ProgramCounterKind {
    ProgramCounterKind::Jump(nnn + self.registers[0] as usize)
  }

  fn run_Cxkk(&mut self, x: usize, kk: u8) -> ProgramCounterKind {    
    let random = generate_random_u8();
    self.registers[x] = random & kk;

    ProgramCounterKind::Next
  }

  fn run_Dxyn(&mut self, x: usize, y: usize, n: usize) -> ProgramCounterKind {
    let vx = self.registers[x] as usize;
    let vy = self.registers[y] as usize;
    let idx = self.index;
    let sprite = &self.memory[idx..idx+n];

    let collision = self.display.draw_sprite(vx, vy, sprite);
    self.registers[0xF] = if collision { 1 } else { 0 };
    
    ProgramCounterKind::Next
  }

  fn run_Ex9E(&mut self, x: usize) -> ProgramCounterKind {
    ProgramCounterKind::skip_if(self.gamepad.is_key_idx_pressed(self.registers[x] as usize))
  }

  fn run_ExA1(&mut self, x: usize) -> ProgramCounterKind {
    ProgramCounterKind::skip_if(!self.gamepad.is_key_idx_pressed(self.registers[x] as usize))
  }
  
  fn run_Fx07(&mut self, x: usize) -> ProgramCounterKind {
    self.registers[x] = self.delay_timer;
    ProgramCounterKind::Next
  }

  fn run_Fx0A(&mut self, x: usize) -> ProgramCounterKind {
    match self.gamepad.get_first_pressed_key_idx() {
      Some(idx) => {
        self.registers[x] = idx as u8;
        ProgramCounterKind::Next
      }
      None => ProgramCounterKind::Jump(self.program_counter)
    }
  }

  fn run_Fx15(&mut self, x: usize) -> ProgramCounterKind {
    self.delay_timer = self.registers[x];
    ProgramCounterKind::Next
  }

  fn run_Fx18(&mut self, x: usize) -> ProgramCounterKind {
    self.sound_timer = self.registers[x];
    ProgramCounterKind::Next
  }

  fn run_Fx1E(&mut self, x: usize) -> ProgramCounterKind {
    self.index += self.registers[x] as usize;
    ProgramCounterKind::Next
  }
  
  fn run_Fx29(&mut self, x: usize) -> ProgramCounterKind {
    self.index = (self.registers[x] as usize) * 5;
    ProgramCounterKind::Next
  }

  fn run_Fx33(&mut self, x: usize) -> ProgramCounterKind {
    let vx = self.registers[x];
    let i = self.index;

    self.memory[i] = vx / 100;
    self.memory[i+1] = (vx / 10) % 10;
    self.memory[i+2] = (vx % 100) % 10;

    ProgramCounterKind::Next
  }
  
  fn run_Fx55(&mut self, x: usize) -> ProgramCounterKind {
    let idx = self.index;

    for i in 0..x+1 {
      self.memory[idx + i] = self.registers[i];
    }
    ProgramCounterKind::Next
  }

  fn run_Fx65(&mut self, x: usize) -> ProgramCounterKind {
    let idx = self.index;

    for i in 0..x+1 {
       self.registers[i] = self.memory[idx + i];
    }
    ProgramCounterKind::Next
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

#[test]
fn testy_test() {
  let total = 266 as u16;

  println!("(total & 0xFF) as u8: {}", (total & 0xFF) as u8);
  println!("total as u8: {}", total as u8); 
  assert!(true)
}