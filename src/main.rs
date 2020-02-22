use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod processor;
use processor::Processor;
mod drivers;
mod font;
use drivers::input::get_keys;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
const PIXEL_SCALE: usize = 10;

fn main() {
    // Initialize graphics context
    // Initialize input handling
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Chip-8 Emulator")
        .with_inner_size(LogicalSize::new(
            (WIDTH * PIXEL_SCALE) as f64,
            (HEIGHT * PIXEL_SCALE) as f64,
        ))
        .build(&event_loop)
        .expect("Could not create window.");

    // Initialize Chip-8 instance
    let mut chip8 = Processor::initialize();
    // Load a program

    let mut keys = [0; 16];

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            state: ElementState::Released,
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode,
                            state,
                            ..
                        },
                    ..
                } => get_keys(virtual_keycode, state, &mut keys),
                _ => (),
            },
            _ => (),
        }
    });
}
