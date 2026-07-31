use osmpbf::{Element, ElementReader};

use crate::{common::import::{import_storage::ImportStorage, tile_index::TileIndex}, parse::index::index};

pub fn parse() -> Result<(), Box<dyn std::error::Error>> {
    let import_storage: ImportStorage = create_import_storage()?;
    let tile_index: TileIndex = index(&import_storage);
    tile_index.save("./tiles");

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
