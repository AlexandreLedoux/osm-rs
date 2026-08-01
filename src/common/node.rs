use serde::{Deserialize, Serialize};

use crate::{common::import_node::ImportNode, utils::utils::coords_to_webmercator};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    id: i64,
    x: f32,
    y: f32,
}

impl Node {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn from_import_node(import_node: &ImportNode) -> Node {
        let (x, y) = coords_to_webmercator(import_node.lat(), import_node.lon());

        Node {
            id: import_node.id(),
            x,
            y,
        }
    }
}
