use serde::{Deserialize, Serialize};

use crate::common::{node::Node, way::Way};

#[derive(Serialize, Deserialize)]
pub struct Tile {
    id: u64,
    pub nodes: Vec<Node>,
    pub ways: Vec<Way>,
}

impl Tile {
    pub fn new(id: u64, nodes: Vec<Node>, ways: Vec<Way>) -> Tile {
        Tile { id, nodes, ways }
    }
}
