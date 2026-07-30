use macroquad::prelude::*;

use std::collections::{HashMap, HashSet};

use crate::{
    load,
    object::{node::Node, storage::Storage},
};

const TILE_SIZE_DEG: f64 = 0.009;
const TILE_SIZE_PIXELS: f64 = 1000.0;

const LOAD_RADIUS: i32 = 2;

#[derive(Clone)]
struct Line {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

struct RenderTile {
    lines: Vec<Line>,
}

#[macroquad::main("Map")]
pub async fn main() {
    let mut storage = Storage::new();

    let mut loaded_tiles: HashSet<(i32, i32)> = HashSet::new();

    let mut render_cache: HashMap<(i32, i32), RenderTile> = HashMap::new();

    let mut camera_lat: f64 = 48.8566;
    let mut camera_lon: f64 = 2.3522;

    let mut zoom: f64 = 1.0;

    let mut last_mouse = mouse_position();

    load_visible_tiles(
        &mut storage,
        &mut loaded_tiles,
        &mut render_cache,
        camera_lat,
        camera_lon,
    );

    loop {
        clear_background(BLACK);

        //
        // déplacement type Google Maps
        //

        let mouse = mouse_position();

        if is_mouse_button_down(MouseButton::Left) {
            let dx = mouse.0 - last_mouse.0;

            let dy = mouse.1 - last_mouse.1;

            let deg_per_pixel = TILE_SIZE_DEG / TILE_SIZE_PIXELS / zoom;

            camera_lon -= dx as f64 * deg_per_pixel;

            camera_lat += dy as f64 * deg_per_pixel;
        }

        last_mouse = mouse;

        //
        // zoom
        //

        let wheel = mouse_wheel().1;

        if wheel != 0.0 {
            zoom += wheel as f64 * 0.15;

            zoom = zoom.clamp(0.2, 20.0);
        }

        load_visible_tiles(
            &mut storage,
            &mut loaded_tiles,
            &mut render_cache,
            camera_lat,
            camera_lon,
        );

        remove_unused_tiles(
            &mut storage,
            &mut loaded_tiles,
            &mut render_cache,
            camera_lat,
            camera_lon,
        );

        draw_map(&render_cache, camera_lat, camera_lon, zoom);

        draw_text(
            &format!(
                "tiles:{} cache:{} zoom:{:.2}",
                loaded_tiles.len(),
                render_cache.len(),
                zoom
            ),
            20.0,
            20.0,
            20.0,
            GREEN,
        );

        next_frame().await;
    }
}

fn load_visible_tiles(
    storage: &mut Storage,
    loaded: &mut HashSet<(i32, i32)>,
    cache: &mut HashMap<(i32, i32), RenderTile>,
    lat: f64,
    lon: f64,
) {
    let tile_x = (lat / TILE_SIZE_DEG).floor() as i32;

    let tile_y = (lon / TILE_SIZE_DEG).floor() as i32;

    for x in tile_x - LOAD_RADIUS..=tile_x + LOAD_RADIUS {
        for y in tile_y - LOAD_RADIUS..=tile_y + LOAD_RADIUS {
            if loaded.contains(&(x, y)) {
                continue;
            }

            if load::load_tile(x, y, storage).is_ok() {
                println!("load tile {} {}", x, y);

                let render = build_render_tile(storage, x, y);

                cache.insert((x, y), render);

                loaded.insert((x, y));
            }
        }
    }
}

fn build_render_tile(storage: &Storage, tile_x: i32, tile_y: i32) -> RenderTile {
    let mut lines = Vec::new();

    let Some(tile) = storage.tiles.get(&(tile_x, tile_y)) else {
        return RenderTile { lines };
    };

    for way_id in tile.ways() {
        let Some(way) = storage.ways.get(way_id) else {
            continue;
        };

        for pair in way.nodes().windows(2) {
            let a = storage.nodes.get(&pair[0]);

            let b = storage.nodes.get(&pair[1]);

            if let (Some(a), Some(b)) = (a, b) {
                lines.push(Line {
                    x1: a.lon(),

                    y1: a.lat(),

                    x2: b.lon(),

                    y2: b.lat(),
                });
            }
        }
    }

    RenderTile { lines }
}

fn draw_map(cache: &HashMap<(i32, i32), RenderTile>, camera_lat: f64, camera_lon: f64, zoom: f64) {
    for tile in cache.values() {
        for line in &tile.lines {
            let (x1, y1) = world_to_screen(line.x1, line.y1, camera_lat, camera_lon, zoom);

            let (x2, y2) = world_to_screen(line.x2, line.y2, camera_lat, camera_lon, zoom);

            if outside_screen(x1, y1, x2, y2) {
                continue;
            }

            draw_line(x1, y1, x2, y2, 1.0, WHITE);
        }
    }
}

fn world_to_screen(lon: f64, lat: f64, camera_lat: f64, camera_lon: f64, zoom: f64) -> (f32, f32) {
    let pixels_per_degree = TILE_SIZE_PIXELS / TILE_SIZE_DEG;

    let x = (lon - camera_lon) * pixels_per_degree * zoom + screen_width() as f64 / 2.0;

    let y = -(lat - camera_lat) * pixels_per_degree * zoom + screen_height() as f64 / 2.0;

    (x as f32, y as f32)
}

fn outside_screen(x1: f32, y1: f32, x2: f32, y2: f32) -> bool {
    let w = screen_width();

    let h = screen_height();

    (x1 < w * -1.0 && x2 < w * -1.0)
        || (x1 > w * 2.0 && x2 > w * 2.0)
        || (y1 < h * -1.0 && y2 < h * -1.0)
        || (y1 > h * 2.0 && y2 > h * 2.0)
}

fn remove_unused_tiles(
    storage: &mut Storage,
    loaded: &mut HashSet<(i32, i32)>,
    cache: &mut HashMap<(i32, i32), RenderTile>,
    lat: f64,
    lon: f64,
) {
    let cx = (lat / TILE_SIZE_DEG).floor() as i32;

    let cy = (lon / TILE_SIZE_DEG).floor() as i32;

    let mut keep = HashSet::new();

    for x in cx - LOAD_RADIUS..=cx + LOAD_RADIUS {
        for y in cy - LOAD_RADIUS..=cy + LOAD_RADIUS {
            keep.insert((x, y));
        }
    }

    let remove: Vec<(i32, i32)> = loaded
        .iter()
        .filter(|t| !keep.contains(t))
        .cloned()
        .collect();

    for tile_id in remove {
        println!("remove tile {} {}", tile_id.0, tile_id.1);

        cache.remove(&tile_id);

        storage.tiles.remove(&tile_id);

        loaded.remove(&tile_id);
    }
}
