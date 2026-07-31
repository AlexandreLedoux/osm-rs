use crate::{
    common::import::{bbox::BBox, import_storage::ImportStorage, tile_index::TileIndex, way::Way}, utils::utils::coords_to_meter,
};

const NB_ZOOM: u8 = 2;

pub fn index(import_storage: &ImportStorage) -> TileIndex {
    let mut tile_index: TileIndex = TileIndex::new();

    for way in import_storage.ways.values() {
        let Some(surface_type) = way.surface_type() else {
            continue;
        };

        for zoom in surface_type.min_zoom()..=NB_ZOOM {
            index_way(way, zoom, import_storage, &mut tile_index);
        }
    }

    tile_index
}

fn index_way(way: &Way, zoom: u8, storage: &ImportStorage, index: &mut TileIndex) {
    let bbox: BBox = way.bbox(storage);

    let (min_x, min_y) = coords_to_meter(bbox.max_lat, bbox.min_lon, zoom);

    let (max_x, max_y) = coords_to_meter(bbox.min_lat, bbox.max_lon, zoom);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            index.add(zoom, x, y, way.id());
        }
    }
}
