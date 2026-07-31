pub struct BBox {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
}

impl BBox {
    pub fn empty() -> Self {
        Self {
            min_lat: f64::MAX,
            max_lat: f64::MIN,
            min_lon: f64::MAX,
            max_lon: f64::MIN,
        }
    }

    pub fn expand(&mut self, lat: f64, lon: f64) {
        self.min_lat = self.min_lat.min(lat);
        self.max_lat = self.max_lat.max(lat);

        self.min_lon = self.min_lon.min(lon);
        self.max_lon = self.max_lon.max(lon);
    }
}
