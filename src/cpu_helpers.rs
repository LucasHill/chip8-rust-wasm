pub enum ProgramCounterKind {
  Next,
  Skip,
  Jump(usize)
}

impl ProgramCounterKind {
  fn skip_if(skip: bool) -> ProgramCounterKind {
    return if skip { ProgramCounterKind::Skip } else { ProgramCounterKind::Next }
  }
}

pub struct OpcodeParts {
  nibbles: (u16, u16, u16, u16),
  x: u16,
  y: u16,
  n: u16,
  nnn: u16,
  kk: u16
}

impl OpcodeParts {
  pub fn new(instruction: u16) -> OpcodeParts {
    let x = (instruction & 0x0F00) >> 8;
    let y = (instruction & 0x00F0) >> 4;
    let n = (instruction & 0x000F);
    let nnn = (instruction & 0x0FFF);
    let kk = (instruction & 0x00FF);

    OpcodeParts {
      nibbles: ((instruction & 0xF000) >> 12, x, y, n),
      x, y, n, nnn, kk
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

