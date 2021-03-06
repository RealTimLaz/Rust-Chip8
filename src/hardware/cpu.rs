use std::{
    ops::Sub,
    time::{Duration, Instant},
};

use super::instruction::Instruction;
use super::{font::FONT, Keyboard};

use rand::Rng;

const HZ_60: f64 = 1.0 / 60.0;
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_WIDTH: usize = 64;

pub struct CPU {
    program_counter: u16,
    memory: [u8; 4096],
    display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    registers: [u8; 16],
    address_register: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    time_when_updated: Instant,
}

impl CPU {
    pub fn new() -> CPU {
        let program_counter = 0x200;

        let mut memory = [0; 4096];

        memory[0x0..0x50].copy_from_slice(&FONT);

        let registers = [0; 16];
        let stack = vec![];
        let address_register = 0;

        CPU {
            program_counter,
            memory,
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            registers,
            address_register,
            stack,
            delay_timer: 0,
            sound_timer: 0,
            time_when_updated: Instant::now(),
        }
    }

    fn flip_pixel(&mut self, sprite_bit: bool, x: u16, y: u16) -> bool {
        let y = y as usize % DISPLAY_HEIGHT;
        let x = x as usize % DISPLAY_WIDTH;

        let pixel = self.display[y][x];

        self.display[y][x] = sprite_bit != pixel;

        pixel && !self.display[y][x]
    }

    fn get_sprite_bit(&self, x: u8, y: u8) -> bool {
        let index = self.address_register + y as u16;
        let row = self.memory[index as usize];

        row.reverse_bits() & (1 << x) != 0
    }

    fn execute(&mut self, instr: Instruction, keyboard: &Keyboard) {
        match instr {
            Instruction::Clear => {
                for row in self.display.iter_mut() {
                    for p in row.iter_mut() {
                        *p = false;
                    }
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
                        did_change = self.flip_pixel(
                            sprite_bit,
                            self.registers[x] as u16 + sprite_x as u16,
                            self.registers[y] as u16 + sprite_y as u16,
                        ) || did_change;
                    }
                }
                self.registers[15] = if did_change { 1 } else { 0 };
            }
            Instruction::KeyEq(reg) => {
                if keyboard.get_key(self.registers[reg]) {
                    self.program_counter += 2;
                }
            }
            Instruction::KeyNeq(reg) => {
                if !keyboard.get_key(self.registers[reg]) {
                    self.program_counter += 2;
                }
            }
            Instruction::GetDelay(reg) => {
                self.registers[reg] = self.delay_timer;
            }
            Instruction::WaitKey(reg) => {
                let key = keyboard.any_key_pressed();
                if key == 0xFF {
                    self.program_counter -= 2;
                }
                self.registers[reg] = key;
            }
            Instruction::SetDelay(reg) => {
                self.delay_timer = self.registers[reg];
                self.time_when_updated = Instant::now();
            }
            Instruction::SetSound(reg) => {
                self.sound_timer = self.registers[reg];
                self.time_when_updated = Instant::now();
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

    fn update_timers(&mut self) {
        // let mut time_since_update = self.time_when_updated.elapsed().as_secs_f64();
        // while time_since_update > HZ_60 {
        //     time_since_update -= HZ_60;
        //     self.delay_timer = self.delay_timer.saturating_sub(1);
        //     self.sound_timer = self.sound_timer.saturating_sub(1);
        // }

        // self.time_when_updated = Instant::now().sub(Duration::from_secs_f64(time_since_update));

        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
    }

    fn draw_display(&self, display: &mut [u8]) {
        for (y, row) in self.display.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                for i in 0..4 {
                    let index = 4 * (y * DISPLAY_WIDTH + x) + i;
                    display[index] = if *pixel { 255 } else { 0 };
                }
            }
        }
    }

    pub fn step(&mut self, display: &mut [u8], keyboard: &Keyboard) {
        // Extract 16 bit instruction code
        let raw_instr_high = self.memory[self.program_counter as usize];
        let raw_instr_low = self.memory[(self.program_counter + 1) as usize];
        let raw_instr = ((raw_instr_high as u16) << 8) | (raw_instr_low as u16);

        // Increment program counter
        self.program_counter += 2;

        // Decode instruction to enum
        let decoded_instr = Instruction::decode(raw_instr);

        // println!(
        //     "PC: {:3X} {:X?} I: {:X} Registers: {:X?}",
        //     self.program_counter - 2,
        //     decoded_instr,
        //     self.address_register,
        //     self.registers
        // );

        // Execute instruction
        self.execute(decoded_instr, keyboard);

        self.update_timers();

        self.draw_display(display);
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.memory[0x200..(0x200 + data.len())].copy_from_slice(data);
    }
}
