use osmpbf::Relation as OsmRelation;

pub struct Relation {
    pub id: i64,
}

impl From<OsmRelation<'_>> for Relation {
    fn from(relation: OsmRelation) -> Self {
        Relation { id: relation.id() }
    }
}
