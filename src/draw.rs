use crate::{pixel::Pixel, color::Color};

pub trait Drawable {
    type IntoIter: IntoIterator<Item = Pixel>;
    fn pixels(&self) -> Self::IntoIter;
}