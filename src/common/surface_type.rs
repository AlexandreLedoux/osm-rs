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
            SurfaceType::SecondaryRoad => 1,
            SurfaceType::ResidentialRoad => 2,
            SurfaceType::Park => 1,
            SurfaceType::Building => 2,
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