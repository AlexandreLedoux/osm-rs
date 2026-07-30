use std::fs;
use std::path::Path;

use crate::object::tile::Tile;

pub fn save_tile(tile_x: i32, tile_y: i32, tile: &Tile) -> Result<(), Box<dyn std::error::Error>> {
    let dir = format!("data/tiles/{}_{}", tile_x, tile_y);

    fs::create_dir_all(&dir)?;

    let nodes_path = Path::new(&dir).join("nodes.bin");

    let ways_path = Path::new(&dir).join("ways.bin");

    let nodes = bincode::serialize(&tile.nodes())?;

    fs::write(nodes_path, nodes)?;

    let ways = bincode::serialize(&tile.ways())?;

    fs::write(ways_path, ways)?;

    Ok(())
}
