use engine::PixelGameEngineBuilder;
use pixel_game_engine::{buffer::Buffer, color::DefaultColors, engine, events::EventLoop, pixel::Pixel};

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
    
    // create engine
    let builder =
    PixelGameEngineBuilder::new(state)
        .with_update(update)
        .with_draw(draw);

    EventLoop::build_and_run(builder)
}
