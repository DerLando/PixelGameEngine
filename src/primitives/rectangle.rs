use crate::{draw::Drawable, pixel::Pixel};

use super::Line;

pub(crate) struct Rectangle {
    pub top_left: Pixel,
    pub width: u32,
    pub height: u32,
}

fn rect_pixels(rect: &Rectangle, filled: bool) -> Vec<Pixel> {
    let corners = vec![
        rect.top_left,
        rect.top_left + (rect.width, 0),
        rect.top_left + (rect.width, rect.height),
        rect.top_left + (0, rect.height),
    ];

    let mut pixels: Vec<Pixel> = Vec::new();

    if !filled {
        // Just draw lines between all corner points
        for (cur, next) in corners
            .iter()
            .zip(corners.iter().skip(1).chain(corners.iter().take(1)))
        {
            pixels.extend(
                Line {
                    from: *cur,
                    to: *next,
                }
                .pixels(),
            );
        }
    } else {
        // Horizontal scan-lines from top left to top right offsetting on each iteration
        for y_offset in 0..rect.height {
            pixels.extend(
                Line {
                    from: corners[0] + (0, y_offset),
                    to: corners[1] + (0, y_offset),
                }
                .pixels(),
            );
        }
    }

    pixels
}

pub(crate) struct HollowRectangle(pub Rectangle);
impl Drawable for HollowRectangle {
    type IntoIter = std::vec::IntoIter<Pixel>;

    fn pixels(&self) -> Self::IntoIter {
        rect_pixels(&self.0, false).into_iter()
    }
}

pub(crate) struct FilledRectangle(pub Rectangle);
impl Drawable for FilledRectangle {
    type IntoIter = std::vec::IntoIter<Pixel>;

    fn pixels(&self) -> Self::IntoIter {
        rect_pixels(&self.0, true).into_iter()
    }
}
