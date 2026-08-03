use std::{
    collections::HashMap,
    error::Error,
    fs::{File, create_dir_all},
    io::Write,
};

use tracing::info;

use crate::common::tile::Tile;

pub fn save(tiles: HashMap<(u8, u32, u32), Tile>) -> Result<(), Box<dyn Error>> {
    for ((zoom, x, y), tile) in tiles {
        let path: String = format!("data/{}/{}/{}/tile", zoom, x, y);

        std::fs::create_dir_all(format!("data/{}/{}/{}", zoom, x, y))?;
        let mut file: File = File::create(path)?;
        let data: Vec<u8> = bincode::serialize(&tile).unwrap();

        file.write_all(&data)?;
    }

    Ok(())
}

pub fn save_tile(zoom: u8, x: u32, y: u32, tile: Tile) -> Result<(), Box<dyn Error>> {
    let dir: String = format!("data/{}/{}/{}", zoom, x, y);
    let path: String = format!("{}/tile", dir);

    create_dir_all(&dir)?;

    let data: Vec<u8> = bincode::serialize(&tile)?;
    let mut file: File = File::create(&path)?;
    file.write_all(&data)?;

    info!("path created: {}", path);

    Ok(())
}