use serde::{Deserialize, Serialize};

use crate::{common::import_node::ImportNode, utils::utils::coords_to_meter};

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    id: i64,
    x: u32,
    y: u32,
}

impl Node {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn from_import_node(import_node: &ImportNode, zoom: u8) -> Node {
        let (x, y) = coords_to_meter(import_node.lat(), import_node.lon(), zoom);

        Node { id: import_node.id(), x, y }
    }
}
