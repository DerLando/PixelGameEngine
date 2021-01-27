use crate::{draw::Drawable, pixel::Pixel};
use bresenham::Bresenham;


pub(crate) struct Line {
    pub from: Pixel,
    pub to: Pixel,
}

impl Drawable for Line {
    type IntoIter = std::vec::IntoIter<Pixel>;

    fn pixels(&self) -> Self::IntoIter {
        Bresenham::new((self.from.x() as isize, self.from.y() as isize), (self.to.x() as isize, self.to.y() as isize))
            .map(|(x, y)| Pixel((x as u32, y as u32)))
            .collect::<Vec<Pixel>>()
            .into_iter()
    }
}