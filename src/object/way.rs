use crate::object::{storage::Storage, tile::Tile};

const TILE_SIZE_METERS: f64 = 1000.0;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Way {
    id: i64,
    nodes: Vec<i64>,
}

impl Way {
    pub fn new(id: i64, nodes: Vec<i64>) -> Way {
        Way {
            id: id,
            nodes: nodes,
        }
    }
}

pub fn add_way(storage: &mut Storage, osm_way: osmpbf::Way) {
    let id: i64 = osm_way.id();

    let node_ids: Vec<i64> = osm_way.refs().collect();

    let way: Way = Way::new(id, node_ids.clone());

    storage.ways.insert(id, way);

    let mut tiles: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();

    for node_id in node_ids {
        if let Some(node) = storage.nodes.get(&node_id) {
            let tile_x = (node.lat() / TILE_SIZE_METERS).floor() as i32;

            let tile_y = (node.lon() / TILE_SIZE_METERS).floor() as i32;

            tiles.insert((tile_x, tile_y));
        }
    }

    for tile_id in tiles {
        storage
            .tiles
            .entry(tile_id)
            .or_insert_with(Tile::new)
            .add_way(id);
    }
}
