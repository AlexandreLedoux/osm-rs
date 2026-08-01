use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum SurfaceType {
    PrimaryRoad,
    SecondaryRoad,
    ResidentialRoad,
    Park,
    Building,
    Water,
}

impl SurfaceType {
    pub fn min_zoom(&self) -> u8 {
        match self {
            SurfaceType::PrimaryRoad => 0,
            SurfaceType::SecondaryRoad => 0,
            SurfaceType::ResidentialRoad => 0,
            SurfaceType::Park => 0,
            SurfaceType::Building => 0,
            SurfaceType::Water => 0,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            SurfaceType::PrimaryRoad => Color::from_rgba(220, 80, 80, 255),
            SurfaceType::SecondaryRoad => Color::from_rgba(240, 160, 80, 255),
            SurfaceType::ResidentialRoad => Color::from_rgba(220, 220, 220, 255),
            SurfaceType::Park => Color::from_rgba(120, 190, 120, 180),
            SurfaceType::Building => Color::from_rgba(180, 160, 140, 255),
            SurfaceType::Water => Color::from_rgba(80, 150, 220, 200),
        }
    }

    pub fn is_polygon(&self) -> bool {
        matches!(
            self,
            SurfaceType::Park | SurfaceType::Building | SurfaceType::Water
        )
    }
}
