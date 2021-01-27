use engine::PixelGameEngineBuilder;
use pixel_game_engine::{buffer::Buffer, color::DefaultColors, engine, events::{EventLoop, KeyEvent}, pixel::Pixel};
use winit::event::VirtualKeyCode;

struct State {
    pub position: Pixel,
    pub velocity: (i32, i32)
}

const RADIUS: u32 = 10;

fn main() -> ! {
    // engine state, we will be animating a simple moving ball
    let state = State {
        position: Pixel((100, 400)),
        velocity: (5, 0)
    };

    // update override
    let update = |s: &mut State| {
        // update position
        s.position = s.position + s.velocity;

        // test if close to border and flip direction vector
        let mut flip = false;
        if s.position.x() + RADIUS >= 800 {
            flip = true;
        }

        if s.position.x() as i32 - RADIUS as i32 <= 0 {
            flip = true;
        }

        if flip {
            s.velocity = (-s.velocity.0, -s.velocity.1);
        }
    };

    // draw override
    let draw = |b: &mut Buffer, s: &State| {
        b.clear(DefaultColors::Black.as_color());
        b.draw_circle(DefaultColors::White.as_color(), s.position, RADIUS, true);
    };

    // key handlers
    let key_movement = |s: &mut State, e: &KeyEvent| {
        match e {
            KeyEvent::Held(k) => match k {
                VirtualKeyCode::W => s.position = s.position + (0, -1),
                VirtualKeyCode::S => s.position = s.position + (0, 1),
                _ => ()
            },
            _ => ()
        }
    };
    
    // create engine
    let builder =
    PixelGameEngineBuilder::new(state)
        .with_update(update)
        .with_draw(draw)
        .add_key_listener(key_movement)
        ;

    EventLoop::build_and_run(builder)
}
