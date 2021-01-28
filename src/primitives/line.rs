use crate::{draw::Drawable, pixel::Pixel};
use bresenham::Bresenham;

pub(crate) struct Line {
    pub from: Pixel,
    pub to: Pixel,
}

impl Drawable for Line {
    type IntoIter = std::vec::IntoIter<Pixel>;

    fn pixels(&self) -> Self::IntoIter {
        Bresenham::new(
            (self.from.x() as isize, self.from.y() as isize),
            (self.to.x() as isize, self.to.y() as isize),
        )
        .map(|(x, y)| Pixel((x as u32, y as u32)))
        .collect::<Vec<Pixel>>()
        .into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vertical_line() {
        // Arrange
        let line = Line {
            from: (10, 10).into(),
            to: (10, 15).into(),
        };

        let expected: Vec<Pixel> = vec![
            (10, 10).into(),
            (10, 11).into(),
            (10, 12).into(),
            (10, 13).into(),
            (10, 14).into(),
            (10, 15).into(),
        ];

        expected
            .into_iter()
            .zip(line.pixels().into_iter())
            .for_each(|(e, a)| assert_eq!(e, a));

        let line = Line {
            from: (10, 500).into(),
            to: (10, 495).into(),
        };

        let expected: Vec<Pixel> = vec![
            (10, 500).into(),
            (10, 499).into(),
            (10, 498).into(),
            (10, 497).into(),
            (10, 496).into(),
            (10, 495).into(),
        ];

        expected
            .into_iter()
            .zip(line.pixels().into_iter())
            .for_each(|(e, a)| assert_eq!(e, a))
    }
}
