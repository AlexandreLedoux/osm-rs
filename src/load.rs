use std::collections::HashMap;
use std::fs;

use crate::object::{node::Node, storage::Storage, tile::Tile, way::Way};

pub fn load_tile(
    tile_x: i32,
    tile_y: i32,
    storage: &mut Storage,
) -> Result<(), Box<dyn std::error::Error>> {
    let dir = format!("data/tiles/{}_{}", tile_x, tile_y);

    let nodes_bytes = fs::read(format!("{}/nodes.bin", dir))?;

    let ways_bytes = fs::read(format!("{}/ways.bin", dir))?;

    let nodes: HashMap<i64, Node> = bincode::deserialize(&nodes_bytes)?;

    let ways: HashMap<i64, Way> = bincode::deserialize(&ways_bytes)?;

    let mut tile = Tile::new();

    for (id, node) in nodes {
        tile.add_node(id);

        storage.nodes.insert(id, node);
    }

    for (id, way) in ways {
        tile.add_way(id);

        storage.ways.insert(id, way);
    }

    storage.add_tile(tile_x, tile_y, tile);

    Ok(())
}
