use osmpbf::{Element, ElementReader};

use crate::{common::import_storage::ImportStorage, import::index::index_and_persist};

pub fn parse() -> Result<(), Box<dyn std::error::Error>> {
    println!("OSM to common");
    let import_storage: ImportStorage = create_import_storage()?;

    println!("Index and index_and_persist");
    index_and_persist(&import_storage)?;

    Ok(())
}

fn create_import_storage() -> Result<ImportStorage, Box<dyn std::error::Error>> {
    let path: &str = "paris_centre.pbf";

    let mut import_storage: ImportStorage = ImportStorage::new();

    let reader = ElementReader::from_path(path)?;

    reader.for_each(|element| {
        if let Element::DenseNode(osm_node) = element {
            import_storage.add_node(osm_node);
        }
    })?;

    println!("Nodes chargés : {}", import_storage.nodes.len());

    let reader = ElementReader::from_path(path)?;

    reader.for_each(|element| {
        if let Element::Way(osm_way) = element {
            import_storage.add_way(osm_way);
        }
    })?;

    println!("Ways chargés : {}", import_storage.ways.len());

    let reader = ElementReader::from_path(path)?;

    reader.for_each(|element| {
        if let Element::Relation(osm_relation) = element {
            import_storage.add_relation(osm_relation);
        }
    })?;

    println!("Relations chargés : {}", import_storage.relations.len());

    Ok(import_storage)
}
