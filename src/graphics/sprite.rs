use raylib::prelude::*;

pub struct Sprite {
    src_rect: Rectangle,
}

impl Sprite {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Sprite {
            src_rect: Rectangle { x, y, width, height}
        }
    }
    
    pub fn draw(&self, pos: Vector2, texture: &Texture2D, d: &mut RaylibDrawHandle) {
        d.draw_texture_rec(texture, self.src_rect, pos, Color::WHITE);
    }
}