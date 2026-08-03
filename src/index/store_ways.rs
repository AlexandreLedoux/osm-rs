use std::error::Error;

pub fn store_ways(filename: &str, zoom: Option<u8>) -> Result<(), Box<dyn Error>> {
    if let Some(z) = zoom {
        store_ways_for_zoom(filename, z)?;
    } else {
        for z in 0..12 {
            store_ways_for_zoom(filename, z)?;
        }
    }

    Ok(())
}

pub fn store_ways_for_zoom(_filename: &str, zoom: u8) -> Result<(), Box<dyn Error>> {
    dbg!(zoom);

    Ok(())
}
