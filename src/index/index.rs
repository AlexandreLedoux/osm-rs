use std::error::Error;

use crate::{common::index_step::IndexStep, index::{index_nodes_coords::index_nodes_coords, store_ways::store_ways}};

pub fn index(filename: String, step: IndexStep, _zoom: Option<u8>) -> Result<(), Box<dyn Error>> {
    match step {
        IndexStep::IndexNodesCoords => index_nodes_coords(filename)?,
        IndexStep::StoreWays => store_ways(&filename)?,
        _ => {}
    }

    Ok(())
}
