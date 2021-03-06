use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub struct Keyboard {
    state: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        let state = [false; 16];

        Keyboard { state }
    }

    pub fn any_key_pressed(&self) -> Option<u8> {
        for (i, key) in self.state.iter().enumerate() {
            if *key {
                return Some(i as u8);
            }
        }
        None
    }

    pub fn handle_input(&mut self, input: &WinitInputHelper) {
        if input.key_pressed(VirtualKeyCode::Key1) {
            self.state[0x1] = true;
        }
        if input.key_pressed(VirtualKeyCode::Key2) {
            self.state[0x2] = true;
        }
        if input.key_pressed(VirtualKeyCode::Key3) {
            self.state[0x3] = true;
        }
        if input.key_pressed(VirtualKeyCode::Key4) {
            self.state[0xC] = true;
        }
        if input.key_pressed(VirtualKeyCode::Q) {
            self.state[0x4] = true;
        }
        if input.key_pressed(VirtualKeyCode::W) {
            self.state[0x5] = true;
        }
        if input.key_pressed(VirtualKeyCode::E) {
            self.state[0x6] = true;
        }
        if input.key_pressed(VirtualKeyCode::R) {
            self.state[0xD] = true;
        }
        if input.key_pressed(VirtualKeyCode::A) {
            self.state[0x7] = true;
        }
        if input.key_pressed(VirtualKeyCode::S) {
            self.state[0x8] = true;
        }
        if input.key_pressed(VirtualKeyCode::D) {
            self.state[0x9] = true;
        }
        if input.key_pressed(VirtualKeyCode::F) {
            self.state[0xE] = true;
        }
        if input.key_pressed(VirtualKeyCode::Z) {
            self.state[0xA] = true;
        }
        if input.key_pressed(VirtualKeyCode::X) {
            self.state[0x0] = true;
        }
        if input.key_pressed(VirtualKeyCode::C) {
            self.state[0xB] = true;
        }
        if input.key_pressed(VirtualKeyCode::V) {
            self.state[0xF] = true;
        }

        if input.key_released(VirtualKeyCode::Key1) {
            self.state[0x1] = false;
        }
        if input.key_released(VirtualKeyCode::Key2) {
            self.state[0x2] = false;
        }
        if input.key_released(VirtualKeyCode::Key3) {
            self.state[0x3] = false;
        }
        if input.key_released(VirtualKeyCode::Key4) {
            self.state[0xC] = false;
        }
        if input.key_released(VirtualKeyCode::Q) {
            self.state[0x4] = false;
        }
        if input.key_released(VirtualKeyCode::W) {
            self.state[0x5] = false;
        }
        if input.key_released(VirtualKeyCode::E) {
            self.state[0x6] = false;
        }
        if input.key_released(VirtualKeyCode::R) {
            self.state[0xD] = false;
        }
        if input.key_released(VirtualKeyCode::A) {
            self.state[0x7] = false;
        }
        if input.key_released(VirtualKeyCode::S) {
            self.state[0x8] = false;
        }
        if input.key_released(VirtualKeyCode::D) {
            self.state[0x9] = false;
        }
        if input.key_released(VirtualKeyCode::F) {
            self.state[0xE] = false;
        }
        if input.key_released(VirtualKeyCode::Z) {
            self.state[0xA] = false;
        }
        if input.key_released(VirtualKeyCode::X) {
            self.state[0x0] = false;
        }
        if input.key_released(VirtualKeyCode::C) {
            self.state[0xB] = false;
        }
        if input.key_released(VirtualKeyCode::V) {
            self.state[0xF] = false;
        }
    }

    pub fn get_key(&self, key: u8) -> bool {
        self.state[key as usize]
    }
}
