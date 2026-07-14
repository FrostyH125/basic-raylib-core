use crate::{graphics::sprite::Sprite, graphics::sprite_animation::SpriteAnimationInstance};
use raylib::prelude::*;

pub struct AnimationData {
    pub frames: &'static [Sprite],
    pub frame_duration: f32,
    pub should_loop: bool,
}

impl AnimationData {
    pub fn update(&self, animation_instance: &mut SpriteAnimationInstance, dt: f32) {
        let frame_count = self.frames.len();

        if animation_instance.finished_playing {
            return;
        }

        if frame_count == 0 {
            animation_instance.finished_playing = true;
            return;
        }

        animation_instance.current_frame_time += dt;

        while animation_instance.current_frame_time >= self.frame_duration {
            animation_instance.current_frame_time -= self.frame_duration;
            animation_instance.current_frame_index += 1;

            if animation_instance.current_frame_index as usize >= frame_count {
                if self.should_loop {
                    animation_instance.current_frame_index = 0;
                } else {
                    animation_instance.current_frame_index = frame_count as u8 - 1;
                    animation_instance.finished_playing = true;
                    break;
                }
            }
        }
    }

    pub fn draw(
        &self,
        animation_instance: &SpriteAnimationInstance,
        d: &mut RaylibDrawHandle,
        pos: Vector2,
        texture: &Texture2D,
    ) {
        self.frames[animation_instance.current_frame_index as usize].draw(d, pos, texture);
    }

    pub fn draw_flp(&self, animation_instance: &SpriteAnimationInstance, d: &mut RaylibDrawHandle, pos: Vector2, texture: &Texture2D, flp_h: bool, flp_v: bool) {
        let sprite_ref = &self.frames[animation_instance.current_frame_index as usize];
        let spr_width = sprite_ref.src_rect.width.ceil();
        let spr_height = sprite_ref.src_rect.height.ceil();
        let flp_h_mult = match flp_h {
            true => -1.0,
            false => 1.0,
        };
        let flp_v_mult = match flp_v {
            true => -1.0,
            false => 1.0,
        };
        let new_sprite = Sprite::new(sprite_ref.src_rect.x as i32, sprite_ref.src_rect.y as i32, (spr_width * flp_h_mult) as i32, (spr_height * flp_v_mult) as i32);

        let rect = Rectangle::new(pos.x, pos.y, spr_width, spr_height);
        new_sprite.draw_pro(d, rect, Vector2::zero(), 0.0, &texture);
    }
}
