use std::collections::HashMap;

use crate::object::{node::Node, tile::Tile, way::Way};

pub struct Storage {
    pub nodes: HashMap<i64, Node>,
    pub ways: HashMap<i64, Way>,
    pub tiles: HashMap<(i32, i32), Tile>,
}
