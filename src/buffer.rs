use pixels::Pixels;
use winit::window::Window;

use crate::{Pixel, color::Color};

pub struct Buffer {
    window: Window,
    pixels: Pixels<Window> 
}

// initialization and field access
impl Buffer {
    pub fn new(window: Window, pixels: Pixels<Window>) -> Self {
        Self {
            window,
            pixels
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn render(&mut self) -> () {
        // TODO: Handle error
        self.pixels.render();
    }
}

/// Drawing routines
impl Buffer {
    /// Converts the given pixel coordinates the the corresponding index
    /// into the raw pixel buffer
    fn pixel_to_index(&self, pixel: Pixel) -> usize {
        ((self.window.inner_size().width + pixel.0) * 4) as usize
    }

    /// Tries to get the raw pixel buffer at the given pixel position.
    /// If the position is out of the window bounds, will return None.
    fn get_pixel(&mut self, pixel: Pixel) -> Option<&mut [u8]> {
        let index = self.pixel_to_index(pixel);

        self.pixels.get_frame().get_mut(index..index + 4)
    }

    /// Draw the given color for the given pixel
    pub fn draw_pixel(&mut self, pixel: Pixel, color: Color) {
        if let Some(raw_pixel) = self.get_pixel(pixel) {
            raw_pixel.copy_from_slice(&color)
        }
    }

    /// Clear the whole window to the given color
    pub fn clear(&mut self, color: Color) {
        for raw_pixel in self.pixels.get_frame().chunks_exact_mut(4) {
            raw_pixel.copy_from_slice(&color)
        }
    }
}