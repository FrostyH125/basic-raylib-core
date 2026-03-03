use raylib::prelude::*;

pub struct Sprite {
    src_rect: Rectangle,
}

impl Sprite {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Sprite {
            src_rect: Rectangle { x, y, width, height },
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, pos: Vector2, texture: &Texture2D) {
        d.draw_texture_rec(texture, self.src_rect, pos, Color::WHITE);
    }

    pub fn draw_pro(
        &self,
        d: &mut RaylibDrawHandle,
        dest_rect: Rectangle,
        origin: Vector2,
        rotation: f32,
        texture: &Texture2D,
    ) {
        d.draw_texture_pro(texture, self.src_rect, dest_rect, origin, rotation, Color::WHITE);
    }
}
