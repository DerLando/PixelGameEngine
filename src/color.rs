/// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
pub type Color = [u8; 4];

#[non_exhaustive]
pub enum DefaultColors {
    Black,
    White
}

impl DefaultColors {
    pub fn as_color(&self) -> Color {
        match *self {
            DefaultColors::Black => [0, 0, 0, 255],
            DefaultColors::White => [255, 255, 255, 255],
        }
    }
}