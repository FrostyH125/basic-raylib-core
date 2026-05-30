use raylib::prelude::*;

pub struct Sprite {
    pub src_rect: Rectangle,
}

const INSET_FOR_SPRITE_BLEED_FIX: f32 = 1.0 / 100.0;

impl Sprite {
    pub const fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Sprite {
            src_rect: Rectangle {
                x: x as f32 + INSET_FOR_SPRITE_BLEED_FIX,
                y: y as f32 + INSET_FOR_SPRITE_BLEED_FIX,
                width: width as f32 - (INSET_FOR_SPRITE_BLEED_FIX),
                height: height as f32 - (INSET_FOR_SPRITE_BLEED_FIX),
            },
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
