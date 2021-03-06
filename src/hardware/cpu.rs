use super::instruction::Instruction;

use rand::Rng;

pub struct CPU {
    program_counter: u16,
    memory: [u8; 4096],
    registers: [u8; 16],
    address_register: u16,
    stack: Vec<u16>,
}

impl CPU {
    pub fn new() -> CPU {
        let program_counter = 0x200;

        let font: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        let mut memory = [0; 4096];

        memory[0x0..0x50].copy_from_slice(&font);

        let registers = [0; 16];
        let stack = vec![];
        let address_register = 0;

        CPU {
            program_counter,
            memory,
            registers,
            address_register,
            stack,
        }
    }

    fn xor_pixel(sprite: bool, x: u8, y: u8, display: &mut [u8]) -> bool {
        let index = (y as usize * 64 + x as usize) * 4;
        let pixel = &mut display[index..index + 4];

        let did_change = pixel[0] == 255 && sprite;

        for p in pixel.iter_mut() {
            *p ^= if sprite { 255 } else { 0 };
        }
        did_change
    }

    fn get_sprite_bit(&self, x: u8, y: u8) -> bool {
        let index = self.address_register + y as u16;
        let row = self.memory[index as usize];

        if x < 8 {
            row.reverse_bits() & (1 << x) != 0
        } else {
            false
        }
    }

    fn execute(&mut self, instr: Instruction, display: &mut [u8]) {
        match instr {
            Instruction::Clear => {
                for byte in display.iter_mut() {
                    *byte = 0;
                }
            }
            Instruction::Return => {
                self.program_counter = match self.stack.pop() {
                    Some(address) => address,
                    None => panic!("Tried to pop empty stack"),
                }
            }
            Instruction::Jump(address) => self.program_counter = address,
            Instruction::Call(address) => {
                self.stack.push(self.program_counter);
                self.program_counter = address;
            }
            Instruction::RegEq(reg, value) => {
                if self.registers[reg] == value {
                    self.program_counter += 2;
                }
            }
            Instruction::RegNeq(reg, value) => {
                if self.registers[reg] != value {
                    self.program_counter += 2;
                }
            }
            Instruction::RegEqReg(reg, other_reg) => {
                if self.registers[reg] == self.registers[other_reg] {
                    self.program_counter += 2;
                }
            }
            Instruction::SetReg(reg, value) => {
                self.registers[reg] = value;
            }
            Instruction::IncReg(reg, value) => {
                self.registers[reg] = self.registers[reg].wrapping_add(value)
            }
            Instruction::RegSetReg(reg, other_reg) => {
                self.registers[reg] = self.registers[other_reg]
            }
            Instruction::Or(reg, other_reg) => self.registers[reg] |= self.registers[other_reg],
            Instruction::And(reg, other_reg) => self.registers[reg] &= self.registers[other_reg],
            Instruction::XOr(reg, other_reg) => self.registers[reg] ^= self.registers[other_reg],
            Instruction::Add(reg, other_reg) => {
                let old_value = self.registers[reg];
                self.registers[reg] = self.registers[reg].wrapping_add(self.registers[other_reg]);
                if self.registers[reg] < old_value {
                    self.registers[15] = 1;
                } else {
                    self.registers[15] = 0;
                }
            }
            Instruction::Sub(reg, other_reg) => {
                let will_borrow = self.registers[reg] < self.registers[other_reg];
                self.registers[reg] = self.registers[reg].wrapping_sub(self.registers[other_reg]);
                self.registers[15] = if will_borrow { 0 } else { 1 };
            }
            Instruction::ShiftR(reg) => {
                let lsb = self.registers[reg] & 0x01;
                self.registers[reg] >>= 1;
                self.registers[15] = lsb;
            }
            Instruction::RevSub(reg, other_reg) => {
                let will_borrow = self.registers[other_reg] < self.registers[reg];
                self.registers[reg] = self.registers[other_reg].wrapping_sub(self.registers[reg]);
                self.registers[15] = if will_borrow { 0 } else { 1 };
            }
            Instruction::ShiftL(reg) => {
                let msb = (self.registers[reg] & 0x80) >> 7;
                self.registers[reg] <<= 1;
                self.registers[15] = msb;
            }
            Instruction::RegNeqReg(reg, other_reg) => {
                if self.registers[reg] != self.registers[other_reg] {
                    self.program_counter += 2;
                }
            }
            Instruction::SetAddress(address) => {
                self.address_register = address;
            }
            Instruction::JumpOffset(address) => {
                self.program_counter = (address + self.registers[0] as u16) % 4096
            }
            Instruction::Random(reg, value) => {
                let mut rng = rand::thread_rng();
                let random_value = rng.gen::<u8>();
                self.registers[reg] = random_value & value;
            }
            Instruction::Draw(x, y, height) => {
                let mut did_change = false;
                for sprite_y in 0..height {
                    for sprite_x in 0..8 {
                        let sprite_bit = self.get_sprite_bit(sprite_x, sprite_y);
                        did_change = did_change
                            || CPU::xor_pixel(
                                sprite_bit,
                                self.registers[x] + sprite_x,
                                self.registers[y] + sprite_y,
                                display,
                            );
                    }
                }
                self.registers[15] = if did_change { 1 } else { 0 };
            }
            Instruction::IncAddress(reg) => {
                self.address_register += self.registers[reg] as u16;
            }
            Instruction::SpriteAddress(reg) => {
                self.address_register = self.registers[reg] as u16 * 5;
            }
            Instruction::RegDump(reg) => {
                for i in 0..=reg {
                    self.memory[(self.address_register + i as u16) as usize] = self.registers[i];
                }
            }
            Instruction::RegLoad(reg) => {
                for i in 0..=reg {
                    self.registers[i] = self.memory[(self.address_register + i as u16) as usize];
                }
            }
            Instruction::BCD(reg) => {
                let val = self.registers[reg];
                self.memory[(self.address_register + 2) as usize] = val % 10;

                let val = val / 10;
                self.memory[(self.address_register + 1) as usize] = val % 10;

                let val = val / 10;
                self.memory[self.address_register as usize] = val % 10;
            }

            _ => (),
        };
    }

    pub fn step(&mut self, display: &mut [u8]) {
        // Extract 16 bit instruction code
        let raw_instr_high = self.memory[self.program_counter as usize];
        let raw_instr_low = self.memory[(self.program_counter + 1) as usize];
        let raw_instr = ((raw_instr_high as u16) << 8) | (raw_instr_low as u16);

        // Increment program counter
        self.program_counter += 2;

        // Decode instruction to enum
        let decoded_instr = Instruction::decode(raw_instr);

        // Execute instruction
        self.execute(decoded_instr, display);
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.memory[0x200..(0x200 + data.len())].copy_from_slice(data);
    }
}
