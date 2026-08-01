use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::{node::Node, way::Way};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    zoom: u8,
    x: u32,
    y: u32,

    id: u64,
    nodes: Vec<Node>,
    ways: Vec<Way>,

    #[serde(skip)]
    node_index: HashMap<i64, usize>,
}

impl Tile {
    pub fn zoom(&self) -> u8 {
        self.zoom
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn ways(&self) -> &[Way] {
        &self.ways
    }

    pub fn add_way(&mut self, way: Way) {
        self.ways.push(way);
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn new(zoom: u8, x: u32, y: u32, id: u64, nodes: Vec<Node>, ways: Vec<Way>) -> Tile {
        let node_index: HashMap<i64, usize> = nodes
            .iter()
            .enumerate()
            .map(|(i, node)| (node.id(), i))
            .collect();

        Tile {
            zoom,
            x,
            y,
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
        let x = self.nodes.iter().map(|n| n.x()).sum::<f32>() / self.nodes.len() as f32;

        let y = self.nodes.iter().map(|n| n.y()).sum::<f32>() / self.nodes.len() as f32;

        (x, y)
    }
}
