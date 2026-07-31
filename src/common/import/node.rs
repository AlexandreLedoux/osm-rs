use osmpbf::DenseNode;

pub struct Node {
    id: i64,
    lat: f64,
    lon: f64,
}

impl Node {
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

impl From<DenseNode<'_>> for Node {
    fn from(node: DenseNode) -> Self {
        Node {
            id: node.id(),
            lat: node.lat(),
            lon: node.lon(),
        }
    }
}
