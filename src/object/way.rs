use crate::object::{storage::Storage, tile::Tile};

use serde::{Deserialize, Serialize};

use std::collections::HashSet;

const TILE_SIZE_DEG: f64 = 0.009;

#[derive(Serialize, Deserialize, Clone)]
pub struct Way {
    id: i64,

    nodes: Vec<i64>,
}

impl Way {
    pub fn new(id: i64, nodes: Vec<i64>) -> Way {
        Way { id, nodes }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn nodes(&self) -> &[i64] {
        &self.nodes
    }
}

pub fn add_way(storage: &mut Storage, osm_way: osmpbf::Way) {
    let id = osm_way.id();

    let node_ids: Vec<i64> = osm_way.refs().collect();

    let way = Way::new(id, node_ids.clone());

    storage.ways.insert(id, way);

    let mut tiles = HashSet::new();

    for node_id in node_ids {
        if let Some(node) = storage.nodes.get(&node_id) {
            let tile_x = (node.lat() / TILE_SIZE_DEG).floor() as i32;

            let tile_y = (node.lon() / TILE_SIZE_DEG).floor() as i32;

            tiles.insert((tile_x, tile_y));
        }
    }

    for tile in tiles {
        storage
            .tiles
            .entry(tile)
            .or_insert_with(Tile::new)
            .add_way(id);
    }
}
