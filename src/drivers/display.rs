use pixels::{wgpu::Surface, Pixels, SurfaceTexture};
use winit::window::Window;

use crate::{HEIGHT, PIXEL_SCALE, WIDTH};

pub struct Display {
    pixels: Pixels,
}

impl Display {
    pub fn new(window: &Window) -> Self {
        let width = (WIDTH * PIXEL_SCALE) as u32;
        let height = (HEIGHT * PIXEL_SCALE) as u32;

        let pixels = {
            let surface = Surface::create(window);
            let surface_texture = SurfaceTexture::new(width, height, surface);
            Pixels::new(width, height, surface_texture).unwrap()
        };

        Display { pixels }
    }

    pub fn draw(&mut self, vram: [u8; 2048]) {
        let frame = self.pixels.get_frame();
        println!("Drawing");
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = 0x00; // R
            pixel[1] = 0x00; // G
            pixel[2] = 0x00; // B
            pixel[3] = 0xff; // A
        }

        self.pixels.render();
    }
}
