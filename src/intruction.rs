use crate::token::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionArg {
    Reg(u8),
    Num(u16),
    Label(u16),
}

pub fn convert_to_chip8(instructions: Vec<(Command, Vec<InstructionArg>)>) -> Vec<u16> {
    let mut new = Vec::new();
    for (cmd, args) in instructions {
        match (cmd, args.as_slice()) {
            (
                Command::ADD,
                &[InstructionArg::Num(n), InstructionArg::Reg(x)]
                | &[InstructionArg::Reg(x), InstructionArg::Num(n)],
            ) => {
                new.push(0x7000 | (x as u16) << 8 | n);
            }
            (Command::ADD, &[InstructionArg::Reg(x), InstructionArg::Reg(y)]) => {
                new.push(0x7000 | (x as u16) << 8 | (y as u16) << 4 | 0x4);
            }
            (
                Command::JMPNE,
                &[InstructionArg::Num(n), InstructionArg::Reg(x)]
                | &[InstructionArg::Reg(x), InstructionArg::Num(n)],
            ) => {
                new.push(0x4000 | (x as u16) << 8 | n);
            }
            (Command::JMPNE, &[InstructionArg::Reg(x), InstructionArg::Reg(y)]) => {
                new.push(0x7000 | (x as u16) << 8 | (y as u16) << 4);
            }
            (Command::RET, &[]) => {
                new.push(0x00EE);
            }
            (Command::JMP, &[InstructionArg::Label(nnn) | InstructionArg::Num(nnn)]) => {
                new.push(0x1000 | (0x200 + 2 * nnn));
            }
            (Command::CALL, &[InstructionArg::Label(nnn) | InstructionArg::Num(nnn)]) => {
                new.push(0x2000 | (0x200 + 2 * nnn));
            }
            _ => unreachable!(),
        }
    }
    new
}
