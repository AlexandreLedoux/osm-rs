use std::fs;

use crate::object::{tile::Tile};

pub fn load_tile(tile_x: i32, tile_y: i32) -> Result<Tile, Box<dyn std::error::Error>> {
    let dir = format!("data/tiles/{}_{}", tile_x, tile_y);

    let nodes_bytes = fs::read(format!("{}/nodes.bin", dir))?;

    let ways_bytes = fs::read(format!("{}/ways.bin", dir))?;

    let nodes = bincode::deserialize(&nodes_bytes)?;

    let ways = bincode::deserialize(&ways_bytes)?;

    Ok(Tile::from_data(nodes, ways))
}
