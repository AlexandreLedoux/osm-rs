use std::{collections::HashSet, error::Error};

use osmpbf::{Element, ElementReader, Way as OsmWay};
use redb::{Database, ReadableDatabase};
use tracing::info;

use crate::{
    common::{tile::Tile, way::Way},
    utils::{load::load_tile, save::save_tile, utils::coords_to_tile},
};

const NODES: redb::TableDefinition<u64, (f64, f64)> = redb::TableDefinition::new("nodes");

pub fn store_ways(filename: &str) -> Result<(), Box<dyn Error>> {
    let db = Database::open("nodes.redb")?;
    let read_txn = db.begin_read()?;
    let nodes = read_txn.open_table(NODES)?;

    let reader = ElementReader::from_path(filename)?;

    reader.for_each(|element| {
        if let Element::Way(osm_way) = element {
            if osm_way.tags().any(|(k, v)| k == "leisure" && v == "park") {
                let name = osm_way
                    .tags()
                    .find(|(k, _)| *k == "name:fr")
                    .map(|(_, v)| v)
                    .or_else(|| osm_way.tags().find(|(k, _)| *k == "name").map(|(_, v)| v))
                    .unwrap_or("<sans nom>");

                info!("Way {} est un parc : {}", osm_way.id(), name);
                
                let way: Way = Way::from(&osm_way);

                for zoom in index_way_at_zoom(&osm_way) {
                    let mut tiles_covered: HashSet<(u32, u32)> = HashSet::new();

                    for node_id in osm_way.refs() {
                        if let Ok(Some(value)) = nodes.get(node_id as u64) {
                            let (lat, lon) = value.value();
                            let (x, y) = coords_to_tile(lat, lon, zoom);

                            tiles_covered.insert((x, y));
                        }
                    }

                    for (x, y) in tiles_covered.clone() {
                        let mut tile: Tile = load_tile(zoom, x, y)
                            .unwrap_or_else(|_| Tile::new(zoom, x, y, Vec::new(), Vec::new()));

                        tile.add_way(way.clone());

                        let _ = save_tile(zoom, x, y, tile);
                    }

                    info!(
                        "Visible zoom: {}, par {} tile(s)",
                        zoom,
                        tiles_covered.len()
                    );
                }
            }
        }
    })?;

    Ok(())
}

fn index_way_at_zoom(osm_way: &OsmWay) -> Vec<u8> {
    if osm_way.tags().any(|(k, v)| k == "leisure" && v == "park") {
        vec![12, 13, 14, 15]
    } else {
        Vec::new()
    }
}
