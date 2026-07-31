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
}