#[derive(Debug, Clone, PartialEq)]
pub enum InstructionArg {
    Reg(u8),
    Num(u16),
    Label(u16),
}
