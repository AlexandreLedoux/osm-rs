use std::collections::HashMap;

use crate::common::render::tile::Tile;

pub struct Storage {
    tiles: HashMap<u64, Tile>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            tiles: HashMap::new(),
        }
    }
}
