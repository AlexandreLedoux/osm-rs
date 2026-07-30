use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::object::{node::Node, storage::Storage, tile::Tile, way::Way};

pub fn save_tile(
    tile_x: i32,
    tile_y: i32,
    storage: &Storage,
    tile: &Tile,
) -> Result<(), Box<dyn std::error::Error>> {
    let dir: String = format!("data/tiles/{}_{}", tile_x, tile_y);

    fs::create_dir_all(&dir)?;

    let nodes_path: std::path::PathBuf = Path::new(&dir).join("nodes.bin");

    let ways_path: std::path::PathBuf = Path::new(&dir).join("ways.bin");

    let mut nodes: HashMap<i64, Node> = HashMap::new();

    for id in tile.nodes() {
        if let Some(node) = storage.nodes.get(id) {
            nodes.insert(*id, node.clone());
        }
    }

    let mut ways: HashMap<i64, Way> = HashMap::new();

    for id in tile.ways() {
        if let Some(way) = storage.ways.get(id) {
            ways.insert(*id, way.clone());
        }
    }

    fs::write(nodes_path, bincode::serialize(&nodes)?)?;

    fs::write(ways_path, bincode::serialize(&ways)?)?;

    Ok(())
}
