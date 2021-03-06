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

fn main() {
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
    //let rom = include_bytes!("maze.ch8");
    //cpu.load_rom(rom);

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
