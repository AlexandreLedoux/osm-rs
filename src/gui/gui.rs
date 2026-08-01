use crate::common::storage::Storage;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut storage: Storage = Storage::new();
    storage.add_tile(0, 0, 0)?;

    println!("Ways dans la tuile : {}", storage.tiles().len());

    Ok(())
}
