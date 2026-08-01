use macroquad::prelude::*;

use crate::{
    common::{camera::Camera, storage::Storage, tile::Tile},
    utils::utils::{coords_to_tile, coords_to_webmercator, webmercator_to_coords},
};

pub async fn run() {
    println!("Chargement du Storage...");

    let lat = 48.864118;
    let lon = 2.325493;

    let osm_zoom: u8 = 14;

    // GPS -> WebMercator
    let (world_x, world_y) = coords_to_webmercator(lat, lon);

    let scale = 5.0 * 2f32.powi(osm_zoom as i32 - 16);

    let mut camera = Camera {
        x: 0.0,
        y: 0.0,
        scale,
    };

    camera.center_on(world_x, world_y);

    let mut storage = Storage::new();

    update_tiles(&mut storage, &camera, osm_zoom);

    loop {
        clear_background(BLACK);

        /*
            Déplacement souris
        */

        if is_mouse_button_down(MouseButton::Left) {
            let delta = mouse_delta_position();

            camera.pan(delta.x, delta.y, osm_zoom);

            update_tiles(&mut storage, &camera, osm_zoom);
        }

        /*
            Affichage
        */

        for tile in storage.tiles().values() {
            draw_tile(tile, &camera);

            draw_tile_border(tile, &camera);
        }

        next_frame().await;
    }
}

fn draw_tile(tile: &Tile, camera: &Camera) {
    for way in tile.ways() {
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

fn draw_tile_border(tile: &Tile, camera: &Camera) {
    let world_size = 40075016.686 / 2f32.powi(tile.zoom() as i32);

    let min_x = tile.x() as f32 * world_size - 20037508.343;

    let max_x = min_x + world_size;

    let max_y = 20037508.343 - tile.y() as f32 * world_size;

    let min_y = max_y - world_size;

    let (x1, y1) = camera.world_to_screen(min_x, min_y);

    let (x2, y2) = camera.world_to_screen(max_x, max_y);

    draw_rectangle_lines(x1, y2, x2 - x1, y1 - y2, 1.0, BLUE);
}

fn update_tiles(storage: &mut Storage, camera: &Camera, zoom: u8) {
    let (world_x, world_y) = camera.world_position();

    let (lat, lon) = webmercator_to_coords(world_x, world_y);

    let (center_x, center_y) = coords_to_tile(lat, lon, zoom);

    let mut needed = Vec::new();

    /*
        Chargement carré 3x3
    */

    for dx in -1i32..=1 {
        for dy in -1i32..=1 {
            let x = center_x as i32 + dx;

            let y = center_y as i32 + dy;

            if x < 0 || y < 0 {
                continue;
            }

            let key = (zoom, x as u32, y as u32);

            needed.push(key);

            if !storage.contains(zoom, x as u32, y as u32) {
                println!("Chargement tile z={} x={} y={}", zoom, x, y);

                if let Err(e) = storage.add_tile(zoom, x as u32, y as u32) {
                    eprintln!("Erreur chargement tile : {}", e);
                }
            }
        }
    }

    /*
        Suppression tiles trop loin
    */

    storage.retain_tiles(|tile| needed.contains(&(tile.zoom(), tile.x(), tile.y())));
}
