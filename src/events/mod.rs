use winit::{event::{Event, VirtualKeyCode}, event_loop::{ControlFlow}};
use winit_input_helper::WinitInputHelper;

use crate::{engine::{PixelGameEngineBuilder}};




mod key_event;

mod mouse_event;

pub struct EventLoop;

impl EventLoop {
    pub fn build_and_run<T: Sized + 'static>(builder: PixelGameEngineBuilder<T>) -> ! {
        // initialize input helper and event loop
        let mut input = WinitInputHelper::new();
        let event_loop = winit::event_loop::EventLoop::new();

        // create an engine from the builder
        let mut engine = builder.build(&event_loop);

        // run the winit loop
        event_loop.run( move |event, _, control_flow| {
            if let Event::RedrawRequested(_) = event {
                engine.draw_frame();
            }
    
            // Handle input events
            if input.update(&event) {
                // Close events
                if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
    
            engine.update();
            })
    }
}