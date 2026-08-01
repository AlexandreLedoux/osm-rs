use osmpbf::Way as OsmWay;
use serde::{Deserialize, Serialize};

use crate::common::{surface_type::SurfaceType, tag::Tag};

#[derive(Serialize, Deserialize, Clone)]
pub struct Way {
    id: i64,
    node_ids: Vec<i64>,
    tags: Vec<Tag>,
}

impl Way {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn node_ids(&self) -> &[i64] {
        &self.node_ids
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn surface_type(&self) -> Option<SurfaceType> {
        for tag in self.tags() {
            match (tag.key(), tag.value()) {
                ("highway", "primary") => {
                    return Some(SurfaceType::PrimaryRoad);
                }

                ("highway", "secondary") => {
                    return Some(SurfaceType::SecondaryRoad);
                }

                ("highway", "residential") => {
                    return Some(SurfaceType::ResidentialRoad);
                }

                ("leisure", "park") => {
                    return Some(SurfaceType::Park);
                }

                ("building", _) => {
                    return Some(SurfaceType::Building);
                }

                _ => {}
            }
        }

        None
    }
}

impl From<OsmWay<'_>> for Way {
    fn from(way: OsmWay) -> Self {
        Way {
            id: way.id(),
            node_ids: way.refs().collect(),
            tags: way
                .tags()
                .map(|(key, value)| Tag::new(key, value))
                .collect(),
        }
    }
}
