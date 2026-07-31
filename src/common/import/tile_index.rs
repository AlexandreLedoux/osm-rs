use std::collections::HashMap;

pub struct TileIndex {
    pub tiles: HashMap<(u8, u32, u32), Vec<i64>>,
}

impl TileIndex {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    pub fn add(&mut self, zoom: u8, x: u32, y: u32, way_id: i64) {
        self.tiles
            .entry((zoom, x, y))
            .or_insert_with(Vec::new)
            .push(way_id);
    }

    pub fn save(&self, root: &str) -> std::io::Result<()> {
        for ((zoom, x, y), way_ids) in &self.tiles {
            let path = format!("{}/{}/{}/data", root, zoom, x, y);

            std::fs::create_dir_all(format!("{}/{}/{}", root, zoom, x))?;

            let mut file = std::fs::File::create(path)?;

            // écrire way_ids en binaire ici
        }

        Ok(())
    }
}
