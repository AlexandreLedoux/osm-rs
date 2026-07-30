use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct Tile {
    nodes: Vec<i64>,
    ways: Vec<i64>,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            nodes: Vec::new(),
            ways: Vec::new(),
        }
    }

    pub fn from_data(nodes: Vec<i64>, ways: Vec<i64>) -> Tile {
        Tile { nodes, ways }
    }

    pub fn add_node(&mut self, id: i64) {
        self.nodes.push(id);
    }

    pub fn add_way(&mut self, id: i64) {
        self.ways.push(id);
    }

    pub fn nodes(&self) -> &[i64] {
        &self.nodes
    }

    pub fn ways(&self) -> &[i64] {
        &self.ways
    }
}
