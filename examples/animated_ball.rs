use engine::PixelGameEngineBuilder;
use pixel_game_engine::{buffer::Buffer, color::DefaultColors, draw::Drawable, engine, events::{EventLoop, KeyEvent}, pixel::Pixel, text::Text};
use winit::event::VirtualKeyCode;

struct State {
    pub position: Pixel,
    pub velocity: (i32, i32),
    pub player_left: Player,
    pub player_right: Player,
}

struct Player {
    pub position: Pixel,
    pub velocity: (f32, f32),
    pub score: u8
}

enum PointWin {
    Left,
    Right
}

const RADIUS: u32 = 5;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const PLAYER_WIDTH: u32 = 20;
const PLAYER_HEIGHT: u32 = 100;
const PLAYER_OFFSET: u32 = 20;
const MAX_PLAYER_VELOCITY: f32 = 2.0;

fn clamp_player_velocity(velocity: (f32, f32)) -> (f32, f32) {
    if velocity.1 >= MAX_PLAYER_VELOCITY {(0.0, MAX_PLAYER_VELOCITY)}
    else {velocity}
}

fn left_player_collides(player: &Player, ball_position: &Pixel) -> bool {
    // early exit, if we are not at the player line
    if ball_position.x() - RADIUS >= player.position.x() + PLAYER_WIDTH {false}
    else {
        if (ball_position.y() - RADIUS > player.position.y() + PLAYER_HEIGHT) |
            (ball_position.y() + RADIUS < player.position.y()) {false}
        else {true}
    }
}

fn right_player_collides(player: &Player, ball_position: &Pixel) -> bool {
    // early exit, if we are not at the player line
    if ball_position.x() + RADIUS <= player.position.x() {false}
    else {
        if (ball_position.y() - RADIUS > player.position.y() + PLAYER_HEIGHT) |
            (ball_position.y() + RADIUS < player.position.y()) {false}
        else {true}
    }
}

fn is_point_win(ball_position: &Pixel) -> Option<PointWin> {
    if ball_position.x() - RADIUS <= PLAYER_OFFSET {return Some(PointWin::Left)}
    if ball_position.x() + RADIUS >= WIDTH - PLAYER_OFFSET {return Some(PointWin::Right)}

    None
}

fn is_player_at_boundary(player: &Player) -> bool {
    if player.position.y() + PLAYER_HEIGHT >= HEIGHT {true}
    else {false}
}

fn main() -> ! {
    // engine state, we will be animating a simple moving ball
    let state = State {
        position: Pixel((HEIGHT / 2, WIDTH / 2)),
        velocity: (-1, -1),
        player_left: Player{
            position: Pixel((PLAYER_OFFSET, 250)),
            velocity: (0.0, 0.0),
            score: 0},
        player_right: Player{
            position: Pixel((800 - PLAYER_OFFSET - PLAYER_WIDTH, 250)),
            velocity: (0.0, 0.0),
            score: 0
        }
    };

    // update override
    let update = |s: &mut State| {
        // update positions
        s.position = s.position + s.velocity;

        s.player_left.velocity = clamp_player_velocity(s.player_left.velocity);
        s.player_left.position = (s.player_left.position.x(), (s.player_left.position.y() as f32 + s.player_left.velocity.1) as u32).into();

        s.player_right.velocity = clamp_player_velocity(s.player_right.velocity);
        s.player_right.position = (s.player_right.position.x(), (s.player_right.position.y() as f32 + s.player_right.velocity.1) as u32).into();

        // test player collision
        if left_player_collides(&s.player_left, &s.position) {
            s.velocity = (-s.velocity.0, s.velocity.1);
            return;
        }

        if right_player_collides(&s.player_right, &s.position) {
            s.velocity = (-s.velocity.0, s.velocity.1);
        }

        // test if a point was won
        if let Some(win) = is_point_win(&s.position) {
            match win {
                PointWin::Left => s.player_left.score += 1,
                PointWin::Right => s.player_right.score += 1,
            }

            s.position = (WIDTH / 2, HEIGHT / 2).into();
            return;
        }

        // test if close to border and flip direction vector
        if s.position.y() + RADIUS >= HEIGHT {
            s.velocity = (s.velocity.0, -s.velocity.1);
        }

        if s.position.y() as i32 - RADIUS as i32 <= 0 {
            s.velocity = (s.velocity.0, -s.velocity.1);
        }

        // test if players are at boundaries
        if is_player_at_boundary(&s.player_left) {s.player_left.velocity = (0.0, 0.0)}
        if is_player_at_boundary(&s.player_right) {s.player_right.velocity = (0.0, 0.0)}
    };

    // draw override
    let draw = |b: &mut Buffer, s: &State| {
        // clear background to black
        b.clear(DefaultColors::Black.as_color());

        // draw the ball
        b.draw_circle(DefaultColors::White.as_color(), s.position, RADIUS, true);

        // draw the left player
        b.draw_rectangle(DefaultColors::White.as_color(), s.player_left.position, PLAYER_WIDTH, PLAYER_HEIGHT, false);

        // draw the left player
        b.draw_rectangle(DefaultColors::White.as_color(), s.player_right.position, PLAYER_WIDTH, PLAYER_HEIGHT, false);

        // draw the score
        let score = format!("{} : {}", s.player_left.score, s.player_right.score);
        b.draw_text(DefaultColors::White.as_color(), (WIDTH / 2 - 50, 2).into(), 50, &score);
    };

    // key handlers
    let key_movement = |s: &mut State, e: &KeyEvent| {
        match e {
            KeyEvent::Held(k) => match k {
                VirtualKeyCode::W => s.player_left.velocity.1 += -0.03,
                VirtualKeyCode::S => s.player_left.velocity.1 += 0.03,
                VirtualKeyCode::Up => s.player_right.velocity.1 += -0.03,
                VirtualKeyCode::Down => s.player_right.velocity.1 += 0.03,
                _ => ()
            },
            _ => ()
        }
    };
    
    // create engine
    let builder =
    PixelGameEngineBuilder::new(state)
        .with_width(WIDTH)
        .with_height(HEIGHT)
        .with_update(update)
        .with_draw(draw)
        .add_key_listener(key_movement)
        ;

    EventLoop::build_and_run(builder)
}
