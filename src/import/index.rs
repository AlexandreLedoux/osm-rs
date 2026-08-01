use std::collections::HashMap;

use crate::{
    common::{import_storage::ImportStorage, node::Node, tile::Tile, way::Way}, utils::{save, utils::coords_to_tile},
};

const NB_ZOOM: u8 = 16;

pub fn index_and_persist(import_storage: &ImportStorage) -> Result<(), Box<dyn std::error::Error>> {
    // On utilise une HashMap pour accumuler les tiles par (zoom, x, y)
    let mut tiles: HashMap<(u8, u32, u32), Tile> = HashMap::new();

    for way in import_storage.ways.values() {
        let Some(surface_type) = way.surface_type() else {
            continue;
        };

        // Pour chaque zoom pertinent pour ce surface_type
        for zoom in surface_type.min_zoom()..=NB_ZOOM {
            // 1. On calcule les tiles (x, y) couvertes par ce way à ce zoom
            let covered_tiles = get_way_tiles(way, zoom, import_storage);

            // 2. On récupère les nodes du way convertis en Node pour ce zoom
            let nodes: Vec<Node> = way
                .node_ids()
                .iter()
                .filter_map(|node_id| {
                    import_storage
                        .nodes
                        .get(node_id)
                        .map(|import_node| Node::from_import_node(import_node))
                })
                .collect();

            // 3. Pour chaque tile couverte, on ajoute le way et ses nodes
            for (x, y) in covered_tiles {
                let entry = tiles
                    .entry((zoom, x, y))
                    .or_insert_with(|| Tile::new(0, Vec::new(), Vec::new()));
                // On évite les doublons de ways et de nodes
                if !entry.ways.iter().any(|w| w.id() == way.id()) {
                    entry.ways.push(way.clone());
                }
                for node in &nodes {
                    if !entry.nodes.iter().any(|n| n.id() == node.id()) {
                        entry.nodes.push(node.clone());
                    }
                }
            }
        }
    }

    println!("Tiles indexées: {}", tiles.len());

    // 4. Persistance des tiles dans l'arborescence de fichiers
    save::save(tiles)?;

    Ok(())
}

/// Calcule les tiles (x, y) couvertes par un way pour un zoom donné.
/// Retourne un Vec<(u32, u32)> représentant les coordonnées (x, y) des tiles à couvrir.
pub fn get_way_tiles(way: &Way, zoom: u8, import_storage: &ImportStorage) -> Vec<(u32, u32)> {
    // 1. Récupérer les nodes du way et les convertir en (x, y) pour le zoom donné
    let points: Vec<(u32, u32)> = way
        .node_ids()
        .iter()
        .filter_map(|node_id| {
            import_storage.nodes.get(node_id).map(|import_node| {
                let (x, y) = coords_to_tile(import_node.lat(), import_node.lon(), zoom);
                (x, y)
            })
        })
        .collect();

    // 2. Si pas de points, retourner un Vec vide
    if points.is_empty() {
        return Vec::new();
    }

    // 3. Calculer la bounding box (min_x, min_y, max_x, max_y)
    let min_x = points.iter().map(|(x, _)| x).min().copied().unwrap_or(0);
    let min_y = points.iter().map(|(_, y)| y).min().copied().unwrap_or(0);
    let max_x = points.iter().map(|(x, _)| x).max().copied().unwrap_or(0);
    let max_y = points.iter().map(|(_, y)| y).max().copied().unwrap_or(0);

    // 4. Générer toutes les tiles (x, y) dans cette bbox
    let mut tiles = Vec::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            tiles.push((x, y));
        }
    }

    tiles
}

/*
fn index_way(way: &Way, zoom: u8, storage: &ImportStorage) {
    let (min_x, min_y) = coords_to_meter(bbox.max_lat, bbox.min_lon, zoom);
    let (max_x, max_y) = coords_to_meter(bbox.min_lat, bbox.max_lon, zoom);

    let points: Vec<(u32, u32)> = way
        .node_ids()
        .iter()
        .map(|node_id| {
            let node = storage.nodes.get(node_id).expect("missing node");
            coords_to_meter(node.lat(), node.lon(), zoom)
        })
        .collect();

    let surface = Surface::new(way.surface_type().unwrap(), points);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            index.add(zoom, x, y, &surface);
        }
    }
}
*/
