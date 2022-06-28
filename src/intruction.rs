use crate::token::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionArg {
    Reg(u8),
    Num(u16),
    Label(u16),
    Chip8(u16),
}

pub fn convert_to_chip8(instructions: Vec<(Command, Vec<InstructionArg>)>) -> Vec<u16> {
    use InstructionArg::*;
    let mut new = Vec::new();
    for (cmd, args) in instructions {
        new.push(match (cmd, args.as_slice()) {
            (Command::ADD, &[Num(nn), Reg(x)] | &[Reg(x), Num(nn)]) => {
                0x7000 | (x as u16) << 8 | nn
            }
            (Command::ADD, &[Reg(x), Reg(y)]) => 0x7004 | (x as u16) << 8 | (y as u16) << 4,
            (Command::JMPNE, &[Num(nn), Reg(x)] | &[Reg(x), Num(nn)]) => {
                0x4000 | (x as u16) << 8 | nn
            }
            (Command::JMPNE, &[Reg(x), Reg(y)]) => 0x7000 | (x as u16) << 8 | (y as u16) << 4,
            (Command::JMPEQ, &[Num(nn), Reg(x)] | &[Reg(x), Num(nn)]) => {
                0x3000 | (x as u16) << 8 | nn
            }
            (Command::JMPEQ, &[Reg(x), Reg(y)]) => 0x5000 | (x as u16) << 8 | (y as u16) << 4,
            (Command::RET, &[]) => 0x00EE,
            (Command::CLR, &[]) => 0x00E0,
            (Command::JMP, &[Label(nnn) | Num(nnn)]) => 0x1000 | (0x200 + 2 * nnn),
            (Command::CALL, &[Label(nnn) | Num(nnn)]) => 0x2000 | (0x200 + 2 * nnn),
            (Command::SET, &[Reg(x), Num(nn)]) => 0x6000 | (x as u16) << 8 | nn,
            (Command::SET, &[Reg(x), Reg(y)]) => 0x8000 | (x as u16) << 8 | (y as u16) << 4,
            (Command::RAND, &[Reg(x), Num(nn)]) => 0xC000 | (x as u16) << 8 | nn,
            (Command::DRAW, &[Reg(x), Reg(y), Num(n)]) => {
                0xD000 | (x as u16) << 8 | (y as u16) << 4 | n
            }
            (Command::OR, &[Reg(x), Reg(y)]) => 0x8001 | (x as u16) << 8 | (y as u16) << 4,
            (Command::AND, &[Reg(x), Reg(y)]) => 0x8002 | (x as u16) << 8 | (y as u16) << 4,
            (Command::XOR, &[Reg(x), Reg(y)]) => 0x8003 | (x as u16) << 8 | (y as u16) << 4,
            (Command::SUB, &[Reg(x), Reg(y)]) => 0x8005 | (x as u16) << 8 | (y as u16) << 4,
            (Command::SHR, &[Reg(x)]) => 0x8006 | (x as u16) << 8,
            (Command::SUBFROM, &[Reg(x), Reg(y)]) => 0x8007 | (x as u16) << 8 | (y as u16) << 4,
            (Command::SHL, &[Reg(x)]) => 0x800E | (x as u16) << 8,
            (Command::POINT, &[Num(nnn) | Label(nnn)]) => 0xA000 | (0x200 + 2 * nnn),
            (Command::OFFJMP, &[Num(nnn) | Label(nnn)]) => 0xB000 | (0x200 + 2 * nnn),
            (Command::SYSCALL, &[Num(nnn)]) => nnn,
            (Command::LOAD, &[Reg(x)]) => 0xF065 | (x as u16) << 8,
            (Command::DUMP, &[Reg(x)]) => 0xF055 | (x as u16) << 8,
            (Command::ADDPTR, &[Reg(x)]) => 0xF01E | (x as u16) << 8,
            (Command::SETPTRCHR, &[Reg(x)]) => 0xF029 | (x as u16) << 8,
            (Command::SETPTRDEC, &[Reg(x)]) => 0xF033 | (x as u16) << 8,
            (Command::GETKEY, &[Reg(x)]) => 0xF00A | (x as u16) << 8,
            (Command::GETDELAY, &[Reg(x)]) => 0xF007 | (x as u16) << 8,
            (Command::SETDELAY, &[Reg(x)]) => 0xF015 | (x as u16) << 8,
            (Command::SETSOUND, &[Reg(x)]) => 0xF018 | (x as u16) << 8,
            (Command::JMPEQKEY, &[Reg(x)]) => 0xE09E | (x as u16) << 8,
            (Command::JMPNEKEY, &[Reg(x)]) => 0xE0A1 | (x as u16) << 8,
            (Command::CHIP, ins) => {
                for ins in ins {
                    if let InstructionArg::Chip8(ins) = ins {
                        new.push(*ins);
                    }
                }
                continue;
            }
            _ => unreachable!(),
        });
    }
    new
}
