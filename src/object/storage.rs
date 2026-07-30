use std::collections::HashMap;

use crate::object::{node::Node, tile::Tile, way::Way};

pub struct Storage {
    pub nodes: HashMap<i64, Node>,

    pub ways: HashMap<i64, Way>,

    pub tiles: HashMap<(i32, i32), Tile>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            nodes: HashMap::new(),
            ways: HashMap::new(),
            tiles: HashMap::new(),
        }
    }

    pub fn add_tile(&mut self, tile_x: i32, tile_y: i32, tile: Tile) {
        self.tiles.insert((tile_x, tile_y), tile);
    }
}
