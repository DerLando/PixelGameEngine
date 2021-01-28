use rusttype::{point, Font, PositionedGlyph, Scale};

use crate::{draw::Drawable, pixel::Pixel};

fn get_font() -> Font<'static> {
    let font_data = include_bytes!("Inter-SemiBold.ttf");
    Font::try_from_bytes(font_data).unwrap()
}

pub struct Text<'a> {
    pub(crate) position: Pixel,
    pub(crate) content: &'a str,
    pub(crate) height: u32
}

impl<'a> Drawable for Text<'a> {
    type IntoIter = std::vec::IntoIter<Pixel>;

    // https://docs.rs/imageproc/0.22.0/src/imageproc/drawing/text.rs.html#52-70
    fn pixels(&self) -> Self::IntoIter {
        let font = get_font();
        let scale = Scale{x: self.height as f32, y: self.height as f32};
        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);

        let glyphs: Vec<PositionedGlyph<'_>> = font.layout(self.content, scale, offset).collect();
        let mut pixels: Vec<Pixel> = Vec::new();

        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, z| {
                    // early exit by coverage treshhold
                    if z < 0.5 {return;}

                    let gx = x as i32 + bb.min.x;
                    let gy = y as i32 + bb.min.y;

                    let pixel_x = gx as u32 + self.position.x();
                    let pixel_y = gy as u32 + self.position.y();

                    pixels.push((pixel_x, pixel_y).into());
                })
            }
        }

        pixels.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_font() {
        let _font = get_font();

        assert!(true);
    }
}