pub fn coords_to_meter(lat: f64, lon: f64, zoom: u8) -> (u32, u32) {
    let n: f64 = 2f64.powi(zoom as i32);
    let x: u32 = ((lon + 180.0) / 360.0 * n).floor() as u32;
    let lat_rad: f64 = lat.to_radians();

    let y: u32 = ((1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / std::f64::consts::PI) / 2.0
        * n)
        .floor() as u32;

    (x, y)
}
