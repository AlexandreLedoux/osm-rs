use crate::common::storage::Storage;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut storage: Storage = Storage::new();
    storage.add_tile(0, 0, 0)?;

    println!("Tiles dans Storage : {}", storage.tiles().len());

    if let Some(way) = storage
        .tiles()
        .values()
        .next()
        .and_then(|tile| tile.ways.first())
    {
        dbg!(way);
    }

    Ok(())
}
