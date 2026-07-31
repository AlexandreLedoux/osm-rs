use crate::common::render::surface_type::SurfaceType;

pub struct Surface {
    surface_type: SurfaceType,
    points: Vec<((f64, f64), (f64, f64))>,
}
