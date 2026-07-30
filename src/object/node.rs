use crate::object::{storage::Storage, tile::Tile};

use serde::{Deserialize, Serialize};

const TILE_SIZE_DEG: f64 = 0.009;

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    id: i64,
    lat: f64,
    lon: f64,
}

impl Node {
    pub fn new(id: i64, lat: f64, lon: f64) -> Node {
        Node { id, lat, lon }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn lat(&self) -> f64 {
        self.lat
    }

    pub fn lon(&self) -> f64 {
        self.lon
    }
}

pub fn add_node(storage: &mut Storage, osm_node: osmpbf::DenseNode) {
    let node = Node::new(osm_node.id(), osm_node.lat(), osm_node.lon());

    storage.nodes.insert(osm_node.id(), node);

    // PAS de conversion mètres ici

    let tile_x = (osm_node.lat() / TILE_SIZE_DEG).floor() as i32;

    let tile_y = (osm_node.lon() / TILE_SIZE_DEG).floor() as i32;

    storage
        .tiles
        .entry((tile_x, tile_y))
        .or_insert_with(Tile::new)
        .add_node(osm_node.id());
}
