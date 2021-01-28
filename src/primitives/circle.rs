use crate::{draw::Drawable, pixel::Pixel};

use super::Line;

pub(crate) struct Circle {
    pub center: Pixel,
    pub radius: u32,
}

/// https://gamedev.stackexchange.com/a/176060
fn circle_pixels(circle: &Circle, is_filled: bool) -> Vec<Pixel> {
    // pixel array
    let mut pixels: Vec<Pixel> = Vec::new();

    let mut d: i32 = (5 - circle.radius as i32 * 4) / 4;
    let mut x = 0i32;
    let mut y = circle.radius as i32;

    loop {
        if !is_filled {
            pixels.push(circle.center + (x, y));
            pixels.push(circle.center + (x, -y));
            pixels.push(circle.center + (-x, y));
            pixels.push(circle.center + (-x, -y));
            pixels.push(circle.center + (y, x));
            pixels.push(circle.center + (y, -x));
            pixels.push(circle.center + (-y, x));
            pixels.push(circle.center + (-y, -x));
        } else {
            pixels.extend(
                Line {
                    from: circle.center + (-x, y),
                    to: circle.center + (x, y),
                }
                .pixels(),
            );
            pixels.extend(
                Line {
                    from: circle.center + (-x, -y),
                    to: circle.center + (x, -y),
                }
                .pixels(),
            );
            pixels.extend(
                Line {
                    from: circle.center + (-y, x),
                    to: circle.center + (y, x),
                }
                .pixels(),
            );
            pixels.extend(
                Line {
                    from: circle.center + (-y, -x),
                    to: circle.center + (y, -x),
                }
                .pixels(),
            );
        }

        // Error correction
        if d < 0 {
            d += 2 * x as i32 + 1;
        } else {
            d += 2 * (x as i32 - y as i32) + 1;
            y -= 1;
        }

        x += 1;

        if x > y {
            break;
        }
    }

    pixels
}

pub(crate) struct HollowCircle(pub Circle);
impl Drawable for HollowCircle {
    type IntoIter = std::vec::IntoIter<Pixel>;

    fn pixels(&self) -> Self::IntoIter {
        circle_pixels(&self.0, false).into_iter()
    }
}

pub(crate) struct FilledCircle(pub Circle);
impl Drawable for FilledCircle {
    type IntoIter = std::vec::IntoIter<Pixel>;

    fn pixels(&self) -> Self::IntoIter {
        circle_pixels(&self.0, true).into_iter()
    }
}
