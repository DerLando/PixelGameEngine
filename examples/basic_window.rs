use engine::PixelGameEngineBuilder;
use pixel_game_engine::{buffer::Buffer, engine};
use winit::{event::{Event, VirtualKeyCode}, event_loop::{ControlFlow, EventLoop}};
use winit_input_helper::WinitInputHelper;

fn main() {
    // create event_loop and input helper
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    // draw override
    let draw = |b: &mut Buffer, s: &u8| b.clear([((*s as f32 / 255.0).sin() * *s as f32) as u8, 0, *s, 255]);
    
    // create engine
    let mut engine =
    PixelGameEngineBuilder::new(0u8)
        .with_update(|s| *s = s.checked_add(1).unwrap_or_default())
        .with_draw(draw)
        .build(&event_loop);

    println!("Created engine!");

    engine.draw_frame();

    event_loop.run(move |event, _, control_flow| {
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

    });
}
