use macroquad::prelude::*;


pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
}


impl Camera {


    pub fn world_to_screen(
        &self,
        x: f32,
        y: f32,
    ) -> (f32, f32) {

        (
            (x - self.x) * self.scale,
            (y - self.y) * self.scale,
        )
    }



    pub fn center_on(
        &mut self,
        x: f32,
        y: f32,
    ) {

        self.x =
            x - screen_width() / 2.0 / self.scale;


        self.y =
            y - screen_height() / 2.0 / self.scale;
    }
}