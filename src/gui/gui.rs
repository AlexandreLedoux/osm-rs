use macroquad::prelude::*;

use crate::{
    common::{camera::Camera, storage::Storage, tile::Tile},
    utils::utils::{coords_to_tile, coords_to_webmercator},
};

pub async fn run() {
    println!("Chargement du Storage...");

    // Position initiale
    let lat = 48.864118; // Paris
    let lon = 2.325493;

    // Zoom des données OSM
    let osm_zoom: u8 = 16;

    // Trouver la tuile contenant le point GPS
    let (tile_x, tile_y) = coords_to_tile(lat, lon, osm_zoom);

    println!(
        "Position GPS {} {} => tile z={} x={} y={}",
        lat, lon, osm_zoom, tile_x, tile_y
    );

    let mut storage = Storage::new();

    // Chargement du carré 3x3 autour de la tuile centrale
    for dx in -1i32..=1 {
        for dy in -1i32..=1 {
            let x = tile_x as i32 + dx;
            let y = tile_y as i32 + dy;

            if x < 0 || y < 0 {
                continue;
            }

            println!("Chargement tile z={} x={} y={}", osm_zoom, x, y);

            if let Err(e) = storage.add_tile(osm_zoom, x as u32, y as u32) {
                eprintln!("Erreur chargement tile {} {} : {}", x, y, e);
            }
        }
    }

    println!("Tiles chargées : {}", storage.tiles().len());

    // Conversion GPS -> coordonnées monde
    let (world_x, world_y) = coords_to_webmercator(lat, lon);

    let mut camera = Camera {
        x: 0.0,
        y: 0.0,
        scale: 0.25,
    };

    // Centrer sur la position GPS
    camera.center_on(world_x as f32, world_y as f32);

    loop {
        clear_background(BLACK);

        // Dessiner toutes les tiles
        for tile in storage.tiles().values() {
            draw_tile(tile, &camera);
        }

        next_frame().await;
    }
}

fn draw_tile(tile: &Tile, camera: &Camera) {
    for way in &tile.ways {
        for node_pair in way.node_ids().windows(2) {
            let Some(a) = tile.node(node_pair[0]) else {
                continue;
            };

            let Some(b) = tile.node(node_pair[1]) else {
                continue;
            };

            let (ax, ay) = camera.world_to_screen(a.x(), a.y());

            let (bx, by) = camera.world_to_screen(b.x(), b.y());

            draw_line(ax, ay, bx, by, 2.0, WHITE);
        }
    }
}
