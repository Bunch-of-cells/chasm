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
                &[InstructionArg::Num(nn), InstructionArg::Reg(x)]
                | &[InstructionArg::Reg(x), InstructionArg::Num(nn)],
            ) => {
                new.push(0x7000 | (x as u16) << 8 | nn);
            }
            (Command::ADD, &[InstructionArg::Reg(x), InstructionArg::Reg(y)]) => {
                new.push(0x7000 | (x as u16) << 8 | (y as u16) << 4 | 0x4);
            }
            (
                Command::JMPNE,
                &[InstructionArg::Num(nn), InstructionArg::Reg(x)]
                | &[InstructionArg::Reg(x), InstructionArg::Num(nn)],
            ) => {
                new.push(0x4000 | (x as u16) << 8 | nn);
            }
            (Command::JMPNE, &[InstructionArg::Reg(x), InstructionArg::Reg(y)]) => {
                new.push(0x7000 | (x as u16) << 8 | (y as u16) << 4);
            }
            (
                Command::JMPEQ,
                &[InstructionArg::Num(nn), InstructionArg::Reg(x)]
                | &[InstructionArg::Reg(x), InstructionArg::Num(nn)],
            ) => {
                new.push(0x3000 | (x as u16) << 8 | nn);
            }
            (Command::JMPEQ, &[InstructionArg::Reg(x), InstructionArg::Reg(y)]) => {
                new.push(0x5000 | (x as u16) << 8 | (y as u16) << 4);
            }
            (Command::RET, &[]) => {
                new.push(0x00EE);
            }
            (Command::CLR, &[]) => {
                new.push(0x00E0);
            }
            (Command::JMP, &[InstructionArg::Label(nnn) | InstructionArg::Num(nnn)]) => {
                new.push(0x1000 | (0x200 + 2 * nnn));
            }
            (Command::CALL, &[InstructionArg::Label(nnn) | InstructionArg::Num(nnn)]) => {
                new.push(0x2000 | (0x200 + 2 * nnn));
            }
            (Command::SET, &[InstructionArg::Reg(x), InstructionArg::Num(nn)]) => {
                new.push(0x6000 | (x as u16) << 8 | nn);
            }
            (Command::SET, &[InstructionArg::Reg(x), InstructionArg::Reg(y)]) => {
                new.push(0x8000 | (x as u16) << 8 | (y as u16) << 4);
            }
            (Command::RAND, &[InstructionArg::Reg(x), InstructionArg::Num(nn)]) => {
                new.push(0xC000 | (x as u16) << 8 | nn);
            }
            _ => unreachable!(),
        }
    }
    new
}
