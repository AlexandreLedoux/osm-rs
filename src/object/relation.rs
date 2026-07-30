pub enum MemberType {
    Node,
    Way,
    Relation,
}

pub struct RelationMember {
    pub member_type: MemberType,
    pub id: i64,
}

pub struct Relation {
    pub id: i64,
    pub members: Vec<RelationMember>,
}