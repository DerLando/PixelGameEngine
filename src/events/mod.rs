use log::error;
use winit::{event::{Event, VirtualKeyCode}, event_loop::{ControlFlow}};
use winit_input_helper::WinitInputHelper;

use crate::{engine::{PixelGameEngineBuilder}};




mod key_event;
pub use key_event::KeyEvent;

mod mouse_event;
pub use mouse_event::MouseEvent;

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
                if engine
                    .draw_frame()
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                    {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
            }
    
            // Handle input events
            if input.update(&event) {
                // Close events, those are static for all implementations
                if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // emit and handle key events
            engine.handle_key_events(emit_key_events(&input));
    
            // call the engines update function, once per loop
            if let Event::MainEventsCleared = event {
                engine.update();
            }

        })
    }
}

fn iter_key_codes() -> impl Iterator<Item = VirtualKeyCode> {
    vec![
        VirtualKeyCode::W,
        VirtualKeyCode::A,
        VirtualKeyCode::S,
        VirtualKeyCode::D,
        VirtualKeyCode::Up,
        VirtualKeyCode::Down,
        VirtualKeyCode::Right,
        VirtualKeyCode::Left,
    ].into_iter()
}

fn emit_key_events(input: &WinitInputHelper) -> impl Iterator<Item = KeyEvent> {
    let mut events: Vec<KeyEvent> = Vec::new();
    iter_key_codes().for_each(|k| {
        if input.key_pressed(k) {events.push(KeyEvent::Pressed(k));};
        if input.key_held(k) {events.push(KeyEvent::Held(k));};
        if input.key_released(k) {events.push(KeyEvent::Released(k));};
    });

    events.into_iter()
}