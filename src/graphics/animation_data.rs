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

    pub fn draw_flp(
        &self,
        animation_instance: &SpriteAnimationInstance,
        d: &mut RaylibDrawHandle,
        pos: Vector2,
        texture: &Texture2D,
        flp_h: bool,
        flp_v: bool,
    ) {
        self.frames[animation_instance.current_frame_index as usize].draw_flp(
            d,
            pos,
            texture,
            flp_h,
            flp_v,
            Color::WHITE,
        );
    }

    pub fn draw_flp_color(
        &self,
        animation_instance: &SpriteAnimationInstance,
        d: &mut RaylibDrawHandle,
        pos: Vector2,
        texture: &Texture2D,
        flp_h: bool,
        flp_v: bool,
        tint: Color,
    ) {
        self.frames[animation_instance.current_frame_index as usize].draw_flp(d, pos, texture, flp_h, flp_v, tint);
    }

    pub fn draw_pro(
        &self,
        animation_instance: &SpriteAnimationInstance,
        d: &mut RaylibDrawHandle,
        dest_rect: Rectangle,
        origin: Vector2,
        rotation: f32,
        texture: &Texture2D,
        color: Color,
    ) {
        self.frames[animation_instance.current_frame_index as usize]
            .draw_pro(d, dest_rect, origin, rotation, texture, color);
    }
}
