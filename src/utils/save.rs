use std::{collections::HashMap, fs::File, io::Write};

use crate::common::tile::Tile;

pub fn save(tiles: HashMap<(u8, u32, u32), Tile>) -> Result<(), Box<dyn std::error::Error>> {
    for ((zoom, x, y), tile) in tiles {
        let path: String = format!("data/{}/{}/{}/tile", zoom, x, y);

        std::fs::create_dir_all(format!("data/{}/{}/{}", zoom, x, y))?;
        let mut file: File = File::create(path)?;
        let data: Vec<u8> = bincode::serialize(&tile).unwrap();

        file.write_all(&data)?;
    }

    Ok(())
}
