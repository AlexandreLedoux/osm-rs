use osmpbf::DenseNode;

pub struct ImportNode {
    id: i64,
    lat: f64,
    lon: f64,
}

impl ImportNode {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn lat(&self) -> f64 {
        self.lat
    }

    pub fn lon(&self) -> f64 {
        self.lon
    }
}

impl From<DenseNode<'_>> for ImportNode {
    fn from(node: DenseNode) -> Self {
        ImportNode {
            id: node.id(),
            lat: node.lat(),
            lon: node.lon(),
        }
    }
}
