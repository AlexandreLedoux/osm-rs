use macroquad::prelude::*;

use std::collections::HashSet;

use crate::{
    load,
    object::{
        node::Node,
        storage::Storage,
    },
};


const TILE_SIZE_DEG: f64 = 0.009;
const TILE_SIZE_PIXELS: f64 = 1000.0;

const LOAD_RADIUS: i32 = 2;



#[macroquad::main("Map")]
pub async fn main() {


    let mut storage = Storage::new();


    let mut loaded_tiles: HashSet<(i32,i32)> = HashSet::new();



    let mut camera_lat: f64 = 48.8566;
    let mut camera_lon: f64 = 2.3522;



    let mut zoom: f64 = 1.0;



    let mut last_mouse = mouse_position();



    load_visible_tiles(
        &mut storage,
        &mut loaded_tiles,
        camera_lat,
        camera_lon
    );



    loop {


        clear_background(BLACK);



        //
        // déplacement carte
        //

        let mouse = mouse_position();


        if is_mouse_button_down(MouseButton::Left) {


            let dx =
                mouse.0 - last_mouse.0;


            let dy =
                mouse.1 - last_mouse.1;



            let deg_per_pixel =
                TILE_SIZE_DEG
                /
                TILE_SIZE_PIXELS
                /
                zoom;



            camera_lon -=
                dx as f64 * deg_per_pixel;



            camera_lat +=
                dy as f64 * deg_per_pixel;
        }


        last_mouse = mouse;



        //
        // zoom
        //

        let wheel = mouse_wheel().1;


        if wheel != 0.0 {


            zoom +=
                wheel as f64 * 0.15;


            zoom =
                zoom.clamp(
                    0.2,
                    20.0
                );
        }




        load_visible_tiles(
            &mut storage,
            &mut loaded_tiles,
            camera_lat,
            camera_lon
        );



        draw_map(
            &storage,
            camera_lat,
            camera_lon,
            zoom
        );



        draw_text(
            &format!(
                "tiles:{} nodes:{} ways:{} zoom:{:.2}",
                loaded_tiles.len(),
                storage.nodes.len(),
                storage.ways.len(),
                zoom
            ),
            20.0,
            20.0,
            20.0,
            GREEN
        );



        next_frame().await;
    }
}







fn load_visible_tiles(
    storage:&mut Storage,
    loaded:&mut HashSet<(i32,i32)>,
    lat:f64,
    lon:f64
) {


    let tile_x =
        (lat / TILE_SIZE_DEG)
        .floor()
        as i32;


    let tile_y =
        (lon / TILE_SIZE_DEG)
        .floor()
        as i32;



    for x in tile_x-LOAD_RADIUS..=tile_x+LOAD_RADIUS {

        for y in tile_y-LOAD_RADIUS..=tile_y+LOAD_RADIUS {


            if loaded.contains(&(x,y)) {
                continue;
            }



            if load::load_tile(
                x,
                y,
                storage
            ).is_ok()
            {

                println!(
                    "load tile {} {}",
                    x,
                    y
                );


                loaded.insert((x,y));
            }
        }
    }
}









fn draw_map(
    storage:&Storage,
    camera_lat:f64,
    camera_lon:f64,
    zoom:f64
) {



    for way in storage.ways.values() {


        for segment in way.nodes().windows(2) {


            let node_a =
                storage.nodes.get(&segment[0]);


            let node_b =
                storage.nodes.get(&segment[1]);



            if let (
                Some(a),
                Some(b)
            ) = (node_a,node_b)
            {


                let (ax,ay)=node_to_pixel(
                    a,
                    camera_lat,
                    camera_lon,
                    zoom
                );


                let (bx,by)=node_to_pixel(
                    b,
                    camera_lat,
                    camera_lon,
                    zoom
                );



                draw_line(
                    ax,
                    ay,
                    bx,
                    by,
                    1.0,
                    WHITE
                );
            }
        }
    }



    // debug nodes

    for node in storage.nodes.values() {


        let (x,y)=node_to_pixel(
            node,
            camera_lat,
            camera_lon,
            zoom
        );


        draw_circle(
            x,
            y,
            2.0,
            RED
        );
    }
}









fn node_to_pixel(
    node:&Node,
    camera_lat:f64,
    camera_lon:f64,
    zoom:f64
)
->(f32,f32)
{


    let pixels_per_degree =
        TILE_SIZE_PIXELS
        /
        TILE_SIZE_DEG;



    let dx =
        node.lon()
        -
        camera_lon;


    let dy =
        node.lat()
        -
        camera_lat;



    let x =
        dx
        *
        pixels_per_degree
        *
        zoom
        +
        screen_width() as f64 / 2.0;



    let y =
        -dy
        *
        pixels_per_degree
        *
        zoom
        +
        screen_height() as f64 / 2.0;



    (
        x as f32,
        y as f32
    )
}