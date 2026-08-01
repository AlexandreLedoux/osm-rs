use std::collections::HashMap;

use crate::{common::tile::Tile, utils::load::load_tile};

pub struct Storage {
    tiles: HashMap<(u8, u32, u32), Tile>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            tiles: HashMap::new(),
        }
    }

    pub fn tiles(&self) -> &HashMap<(u8, u32, u32), Tile> {
        &self.tiles
    }

    pub fn add_tile(&mut self, zoom: u8, x: u32, y: u32) -> Result<(), Box<dyn std::error::Error>> {
        let tile: Tile = load_tile(zoom, x, y)?;
        self.tiles.insert((zoom, x, y), tile);
        Ok(())
    }

    pub fn remove_tile(&mut self, zoom: u8, x: u32, y: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.tiles.remove(&(zoom, x, y));

        Ok(())
    }
}
