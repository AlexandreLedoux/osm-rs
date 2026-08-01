use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::{node::Node, way::Way};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    id: u64,
    pub nodes: Vec<Node>,
    pub ways: Vec<Way>,

    #[serde(skip)]
    node_index: HashMap<i64, usize>,
}

impl Tile {
    pub fn new(id: u64, nodes: Vec<Node>, ways: Vec<Way>) -> Tile {
        let node_index: HashMap<i64, usize> = nodes
            .iter()
            .enumerate()
            .map(|(i, node)| (node.id(), i))
            .collect();

        Tile {
            id,
            nodes,
            ways,
            node_index,
        }
    }

    pub fn node_index(&self) -> &HashMap<i64, usize> {
        &self.node_index
    }

    pub fn node(&self, id: i64) -> Option<&Node> {
        self.node_index.get(&id).map(|&index| &self.nodes[index])
    }

    pub fn rebuild_index(&mut self) {
        self.node_index = self
            .nodes
            .iter()
            .enumerate()
            .map(|(i, node)| (node.id(), i))
            .collect();
    }

    pub fn center(&self) -> (f32, f32) {
        let x = self.nodes.iter()
            .map(|n| n.x())
            .sum::<f32>() / self.nodes.len() as f32;

        let y = self.nodes.iter()
            .map(|n| n.y())
            .sum::<f32>() / self.nodes.len() as f32;

        (x, y)
    }
}
