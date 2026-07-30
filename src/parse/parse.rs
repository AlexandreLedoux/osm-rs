use osmpbf::{Element, ElementReader};

use crate::object::{node::add_node, storage::Storage, way::add_way};

pub fn parse() -> Result<Storage, Box<dyn std::error::Error>> {
    let path = "ile-de-france-260729.osm.pbf";

    let mut storage = Storage::new();

    /*
        PASS 1 :
        On charge tous les nodes
    */

    let reader = ElementReader::from_path(path)?;

    reader.for_each(|element| {
        if let Element::DenseNode(node) = element {
            add_node(&mut storage, node);
        }
    })?;

    println!("Nodes chargés : {}", storage.nodes.len());

    /*
        PASS 2 :
        On charge les ways maintenant que
        tous les nodes existent
    */

    let reader = ElementReader::from_path(path)?;

    reader.for_each(|element| {
        if let Element::Way(way) = element {
            add_way(&mut storage, way);
        }
    })?;

    println!("Ways chargés : {}", storage.ways.len());

    println!("Tiles : {}", storage.tiles.len());

    Ok(storage)
}
