use std::collections::HashMap;

use osmpbf::{Element, ElementReader};

use crate::object::{node::add_node, storage::Storage, way::Way};

pub fn parse() -> Result<Storage, Box<dyn std::error::Error>> {
    let reader = ElementReader::from_path("ile-de-france-260729.osm.pbf")?;

    let mut storage: Storage = Storage {
        nodes: HashMap::new(),
        ways: HashMap::new(),
        tiles: HashMap::new(),
    };

    reader.for_each(|element| match element {
        Element::DenseNode(node) => {
            add_node(&mut storage, node);
        }

        Element::Way(way) => {
            let nodes: Vec<i64> = way.refs().collect();

            let new_way = Way::new(way.id(), nodes);

            storage.ways.insert(way.id(), new_way);
        }

        _ => {}
    })?;

    println!("Nodes: {}", storage.nodes.len());

    println!("Tiles: {}", storage.tiles.len());

    Ok(storage)
}
