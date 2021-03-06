#[derive(Debug)]
pub enum Instruction {
    Clear,
    Return,
    Jump(u16),
    Call(u16),
    RegEq(usize, u8),
    RegNeq(usize, u8),
    RegEqReg(usize, usize),
    SetReg(usize, u8),
    IncReg(usize, u8),
    RegSetReg(usize, usize),
    Or(usize, usize),
    And(usize, usize),
    XOr(usize, usize),
    Add(usize, usize),
    Sub(usize, usize),
    ShiftR(usize),
    RevSub(usize, usize),
    ShiftL(usize),
    RegNeqReg(usize, usize),
    SetAddress(u16),
    JumpOffset(u16),
    Random(usize, u8),
    Draw(usize, usize, u8),
    KeyEq(usize),
    KeyNeq(usize),
    GetDelay(usize),
    WaitKey(usize),
    SetDelay(usize),
    SetSound(usize),
    IncAddress(usize),
    SpriteAddress(usize),
    RegDump(usize),
    RegLoad(usize),
    BCD(usize),
    NoOp,
}

impl Instruction {
    pub fn decode(opcode: u16) -> Instruction {
        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            opcode & 0x000F,
        );

        let address = opcode & 0x0FFF;
        let x_register = ((opcode & 0x0F00) >> 8) as usize;
        let y_register = ((opcode & 0x00F0) >> 4) as usize;
        let value = (opcode & 0x00FF) as u8;
        let short_value = (opcode & 0x000F) as u8;

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => Instruction::Clear,
            (0x0, 0x0, 0xE, 0xE) => Instruction::Return,
            (0x1, _, _, _) => Instruction::Jump(address),
            (0x2, _, _, _) => Instruction::Call(address),
            (0x3, _, _, _) => Instruction::RegEq(x_register, value),
            (0x4, _, _, _) => Instruction::RegNeq(x_register, value),
            (0x5, _, _, 0) => Instruction::RegEqReg(x_register, y_register),
            (0x6, _, _, _) => Instruction::SetReg(x_register, value),
            (0x7, _, _, _) => Instruction::IncReg(x_register, value),
            (0x8, _, _, 0x0) => Instruction::RegSetReg(x_register, y_register),
            (0x8, _, _, 0x1) => Instruction::Or(x_register, y_register),
            (0x8, _, _, 0x2) => Instruction::And(x_register, y_register),
            (0x8, _, _, 0x3) => Instruction::XOr(x_register, y_register),
            (0x8, _, _, 0x4) => Instruction::Add(x_register, y_register),
            (0x8, _, _, 0x5) => Instruction::Sub(x_register, y_register),
            (0x8, _, _, 0x6) => Instruction::ShiftR(x_register),
            (0x8, _, _, 0x7) => Instruction::RevSub(x_register, y_register),
            (0x8, _, _, 0xE) => Instruction::ShiftL(x_register),
            (0x9, _, _, 0x0) => Instruction::RegNeqReg(x_register, y_register),
            (0xA, _, _, _) => Instruction::SetAddress(address),
            (0xB, _, _, _) => Instruction::JumpOffset(address),
            (0xC, _, _, _) => Instruction::Random(x_register, value),
            (0xD, _, _, _) => Instruction::Draw(x_register, y_register, short_value),
            (0xE, _, 0x9, 0xE) => Instruction::KeyEq(x_register),
            (0xE, _, 0xA, 0x1) => Instruction::KeyNeq(x_register),
            (0xF, _, 0x0, 0x7) => Instruction::GetDelay(x_register),
            (0xF, _, 0x0, 0xA) => Instruction::WaitKey(x_register),
            (0xF, _, 0x1, 0x5) => Instruction::SetDelay(x_register),
            (0xF, _, 0x1, 0x8) => Instruction::SetSound(x_register),
            (0xF, _, 0x1, 0xE) => Instruction::IncAddress(x_register),
            (0xF, _, 0x2, 0x9) => Instruction::SpriteAddress(x_register),
            (0xF, _, 0x3, 0x3) => Instruction::BCD(x_register),
            (0xF, _, 0x5, 0x5) => Instruction::RegDump(x_register),
            (0xF, _, 0x6, 0x5) => Instruction::RegLoad(x_register),
            _ => {
                println!("Unknown Instruction: {:X}", opcode);
                Instruction::NoOp
            }
        }
    }
}
