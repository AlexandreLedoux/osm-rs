use std::collections::HashMap;

use osmpbf::{DenseNode as OsmNode, Relation as OsmRelation, Way as OsmWay};

use crate::common::{import_node::ImportNode, relation::Relation, way::Way};

pub struct ImportStorage {
    pub nodes: HashMap<i64, ImportNode>,
    pub ways: HashMap<i64, Way>,
    pub relations: HashMap<i64, Relation>,
}

impl ImportStorage {
    pub fn new() -> ImportStorage {
        ImportStorage {
            nodes: HashMap::new(),
            ways: HashMap::new(),
            relations: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, osm_node: OsmNode) {
        self.nodes.insert(osm_node.id(), ImportNode::from(osm_node));
    }

    pub fn add_way(&mut self, osm_way: OsmWay) {
        self.ways.insert(osm_way.id(), Way::from(&osm_way));
    }

    pub fn add_relation(&mut self, osm_relation: OsmRelation) {
        self.relations
            .insert(osm_relation.id(), Relation::from(osm_relation));
    }
}
