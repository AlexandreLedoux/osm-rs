use macroquad::color::{Color, DARKGRAY, GREEN};

use crate::common::way::Way;

#[derive(Debug, Clone)]
pub enum Geometry {
    Polygon,
    Line { width: f32 },
}

#[derive(Debug, Clone)]
pub struct MapObject {
    pub color: Color,
    pub geometry: Geometry,
}

impl MapObject {
    pub fn new(color: Color, geometry: Geometry) -> MapObject {
        MapObject { color, geometry }
    }

    pub fn from_way(way: &Way) -> Option<MapObject> {
        for tag in way.tags() {
            match (tag.key(), tag.value()) {
                ("highway", "primary") => {
                    return Some(MapObject::new(DARKGRAY, Geometry::Line { width: 10.0 }));
                }

                ("leisure", "park") => {
                    return Some(MapObject::new(GREEN, Geometry::Polygon));
                }

                _ => {}
            }
        }

        None
    }
}
