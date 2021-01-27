use std::{collections::binary_heap::Iter, convert::TryInto, marker::PhantomData, time::Instant};

use log::error;
use pixels::{Error, Pixels, PixelsContext, SurfaceTexture};
use winit::{dpi::LogicalSize, window::Window};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::{pixel::Pixel, buffer::Buffer, color::{Color, DefaultColors}};

pub struct PixelGameEngineBuilder<T>
where T: Sized
{
    state: T,
    width: u32,
    height: u32,
    update_fn: Box<dyn FnMut(&mut T) -> ()>,
    draw_fn: Box<dyn FnMut(&mut Buffer, &T) -> ()>,
}

impl<T> PixelGameEngineBuilder<T>
where T: Sized
{
    pub fn new(state: T) -> Self {
        Self {
            state,
            width: 800,
            height: 600,
            update_fn: Box::new(|_| ()),
            draw_fn: Box::new(|_, _| ()),
        }
    }

    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn with_update(mut self, update_fn : impl FnMut(&mut T) -> () + 'static) -> Self {
        self.update_fn = Box::new(update_fn);
        self
    }

    pub fn with_draw(mut self, draw_fn: impl FnMut(&mut Buffer, &T) -> () + 'static) -> Self {
        self.draw_fn = Box::new(draw_fn);
        self
    }

    pub fn build(self, event_loop: &EventLoop<()>) -> PixelGameEngine<T> {
        PixelGameEngine::new(self.state, self.width, self.height, self.update_fn, self.draw_fn, event_loop)
    }
}

pub struct PixelGameEngine<T>
where 
T: Sized,
{
    state: T,
    buffer: Buffer,
    update_fn: Box<dyn FnMut(&mut T) -> ()>,
    draw_fn: Box<dyn FnMut(&mut Buffer, &T) -> ()>,
}

impl<T> PixelGameEngine<T>
where T: Sized {
    fn new(
        state: T,
        width: u32, 
        height: u32, 
        update_fn: Box<dyn FnMut(&mut T) -> ()>, 
        draw_fn: Box<dyn FnMut(&mut Buffer, &T) -> ()>, 
        event_loop: &EventLoop<()>
    ) -> Self {
        // initialize logger
        env_logger::init();
        
        // create the window
        let window = {
            let size = LogicalSize::new(width, height);
            WindowBuilder::new()
                .with_title("Hello PixelGameEngine")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        // create the inner pixel buffer
        let pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(width, height, surface_texture).unwrap()
        };

        // return self
        Self {
            state,
            buffer: Buffer::new(window, pixels),
            update_fn,
            draw_fn
        }
    }

    pub fn state(&self) -> &T {
        &self.state
    }

    /// Updates the engine state by calling it's update_fn.
    /// After the update, a redraw will be requested from the event loop.
    pub fn update(&mut self) {
        // update the inner state
        (self.update_fn)(&mut self.state);

        self.buffer.window().request_redraw();
    }
}

// Drawing routines
impl<T> PixelGameEngine<T>
where T: Sized {
    /// Draw the engine state to a new frame.
    /// The default implementation of this does nothing.
    pub fn draw_frame(&mut self) {
        // draw via draw fn
        (self.draw_fn)(&mut self.buffer, &self.state);

        // render pixels buffer
        self.buffer.render();
    }
}