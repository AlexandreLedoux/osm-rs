pub fn coords_to_tile(lat: f64, lon: f64, zoom: u8) -> (u32, u32) {
    let n: f64 = 2f64.powi(zoom as i32);
    let x: u32 = ((lon + 180.0) / 360.0 * n).floor() as u32;
    let lat_rad: f64 = lat.to_radians();

    let y: u32 = ((1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / std::f64::consts::PI) / 2.0
        * n)
        .floor() as u32;

    (x, y)
}

pub fn coords_to_webmercator(lat: f64, lon: f64) -> (f32, f32) {
    const R: f64 = 6378137.0; // Rayon de la Terre (WGS84)

    let x = R * lon.to_radians();

    let y = R
        * ((std::f64::consts::PI / 4.0) + (lat.to_radians() / 2.0))
            .tan()
            .ln();

    (x as f32, y as f32)
}