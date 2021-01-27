#![deny(clippy::all)]
#![forbid(unsafe_code)]

pub mod engine;
pub mod color;
pub mod buffer;

pub type Pixel = (u32, u32);
