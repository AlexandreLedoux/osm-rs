use crate::common::render::surface::Surface;

pub struct Tile {
    id: u64,
    x: u32,
    y: u32,
    surface: Vec<Surface>,
}