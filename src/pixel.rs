use std::ops::{Add, Sub};


#[derive(Clone, Copy, Debug)]
pub struct Pixel(pub(u32, u32));

impl Pixel {
    pub fn x(&self) -> u32 {self.0.0}
    pub fn y(&self) -> u32 {self.0.1}
}

impl From<(u32, u32)> for Pixel {
    fn from(values: (u32, u32)) -> Self {
        Pixel(values)
    }
}

impl Add<Pixel> for Pixel {
    type Output = Pixel;

    fn add(self, rhs: Pixel) -> Self::Output {
        Pixel((self.x() + rhs.x(), self.y() + rhs.y()))
    }
}

impl Sub<Pixel> for Pixel {
    type Output = Pixel;

    fn sub(self, rhs: Pixel) -> Self::Output {
        let x = self.x().checked_sub(rhs.x()).unwrap_or(0);
        let y = self.y().checked_sub(rhs.y()).unwrap_or(0);

        Pixel((x, y))
    }
}

impl Add<(u32, u32)> for Pixel {
    type Output = Pixel;

    fn add(self, rhs: (u32, u32)) -> Self::Output {
        self + Pixel(rhs)
    }
}

impl Sub<(u32, u32)> for Pixel {
    type Output = Pixel;

    fn sub(self, rhs: (u32, u32)) -> Self::Output {
        self - Pixel(rhs)
    }
}

impl Add<(i32, i32)> for Pixel {
    type Output = Pixel;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        fn clamped_add(lhs: u32, rhs: i32) -> u32 {
            (lhs as i32 + rhs).max(0) as u32
        }

        Pixel((clamped_add(self.x(), rhs.0), clamped_add(self.y(), rhs.1)))
    }
}