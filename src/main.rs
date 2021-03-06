use std::{env, fs::File, io::Read};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod hardware;

use self::hardware::CPU;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 320;

fn read_rom_from_file(rom_name: &str) -> Vec<u8> {
    let mut f =
        File::open(rom_name).unwrap_or_else(|_| panic!("Unable to open file: {}", rom_name));
    let mut rom = Vec::new();
    f.read_to_end(&mut rom).expect("Unable to read rom");

    rom
}

fn main() {
    // Read ROM data
    let rom_name = env::args().nth(1).expect("No file name given for ROM");
    let rom_data = read_rom_from_file(&rom_name);

    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Chip-8 Emulator")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(64, 33, surface_texture).unwrap()
    };

    let mut cpu = CPU::new();
    cpu.load_rom(&rom_data);

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        cpu.step(pixels.get_frame());

        window.request_redraw();
    });
}
