use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum SurfaceType {
    PrimaryRoad,
    SecondaryRoad,
    ResidentialRoad,
    Park,
    Building,
}

impl SurfaceType {
    pub fn min_zoom(&self) -> u8 {
        match self {
            SurfaceType::PrimaryRoad => 0,
            SurfaceType::SecondaryRoad => 0,
            SurfaceType::ResidentialRoad => 0,
            SurfaceType::Park => 0,
            SurfaceType::Building => 0,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            SurfaceType::PrimaryRoad => "primary_road".to_string(),
            SurfaceType::SecondaryRoad => "secondary_road".to_string(),
            SurfaceType::ResidentialRoad => "residential_road".to_string(),
            SurfaceType::Park => "park".to_string(),
            SurfaceType::Building => "building".to_string(),
        }
    }
}