pub fn lat_lon_to_meters(lat: f64, lon: f64) -> (f64, f64) {
    let earth_radius: f64 = 6378137.0;

    let x: f64 = earth_radius * lon.to_radians();

    let y: f64 = earth_radius
        * ((std::f64::consts::PI / 4.0 + lat.to_radians() / 2.0)
        .tan())
        .ln();

    (x, y)
}