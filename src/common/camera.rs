use macroquad::prelude::*;

pub struct Camera {
    pub x: f32,

    pub y: f32,

    pub scale: f32,
}

impl Camera {
    pub fn world_to_screen(&self, x: f32, y: f32) -> (f32, f32) {
        ((x - self.x) * self.scale, -(y - self.y) * self.scale)
    }

    pub fn center_on(&mut self, x: f32, y: f32) {
        self.x = x - screen_width() / 2.0 / self.scale;

        self.y = y + screen_height() / 2.0 / self.scale;
    }

    pub fn pan(&mut self, dx: f32, dy: f32, zoom: u8) {
        /*
            Plus le zoom est élevé,
            plus on doit être précis.

            Zoom 14 = base
            Zoom 16 = déplacement divisé par 4
            Zoom 12 = déplacement multiplié par 4
        */

        let zoom_factor = 2f32.powi(14 - zoom as i32);

        let speed = 300.0 * zoom_factor;

        /*
            Sens Google Maps :

            souris droite -> carte droite
            souris haut -> carte haut
        */

        self.x += dx * speed / self.scale;

        self.y -= dy * speed / self.scale;
    }

    pub fn world_position(&self) -> (f32, f32) {
        (
            self.x + screen_width() / 2.0 / self.scale,
            self.y - screen_height() / 2.0 / self.scale,
        )
    }
}
