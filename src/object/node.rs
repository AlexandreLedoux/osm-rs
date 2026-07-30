use crate::{object::{storage::Storage, tile::Tile}, utils::utils::lat_lon_to_meters};

const TILE_SIZE_METERS: f64 = 1000.0;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Node {
    id: i64,
    lat: f64,
    lon: f64,
}

impl Node {
    pub fn new(id: i64, lat: f64, lon: f64) -> Node {
        Node {
            id: id,
            lat: lat,
            lon: lon,
        }
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
    let (x, y) = lat_lon_to_meters(osm_node.lat(), osm_node.lon());

    let id = osm_node.id();

    let node = Node::new(id, x, y);

    // stockage global
    storage.nodes.insert(id, node);

    // index spatial
    let tile_x = (x / TILE_SIZE_METERS).floor() as i32;
    let tile_y = (y / TILE_SIZE_METERS).floor() as i32;

    storage
        .tiles
        .entry((tile_x, tile_y))
        .or_insert_with(Tile::new)
        .add_node(id);
}
