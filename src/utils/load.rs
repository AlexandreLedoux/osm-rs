use crate::common::tile::Tile;

pub fn load_tile(zoom: u8, x: u32, y: u32) -> Result<Tile, Box<dyn std::error::Error>> {
    let path: String = format!("data/{}/{}/{}/tile", zoom, x, y);

    let bytes: Vec<u8> = std::fs::read(path)?;

    let surfaces: Tile = bincode::deserialize(&bytes)?;

    Ok(surfaces)
}
