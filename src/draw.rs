use crate::{pixel::Pixel};

pub trait Drawable {
    type IntoIter: IntoIterator<Item = Pixel>;
    fn pixels(&self) -> Self::IntoIter;
}