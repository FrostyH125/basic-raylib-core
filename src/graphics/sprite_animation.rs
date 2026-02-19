use raylib::prelude::*;

use crate::graphics::animation_data::AnimationData;

pub struct SpriteAnimationInstance {
    current_frame_time: f32,
    current_frame_index: u8,
    pub can_play: bool,
}

impl SpriteAnimationInstance {
    pub fn new() -> Self {
        SpriteAnimationInstance {
            current_frame_time: 0.0,
            current_frame_index: 0,
            can_play: true,
        }
    }

    pub fn update(&mut self, animation_data: &AnimationData, dt: f32) {
        let frame_count = animation_data.frames.len() as u8;

        if frame_count == 0 {
            self.can_play = false;
            return;
        }

        if !self.can_play {
            return;
        }

        self.current_frame_time += dt;

        while self.current_frame_time >= animation_data.frame_duration {
            self.current_frame_time -= animation_data.frame_duration;
            self.current_frame_index += 1;

            if self.current_frame_index >= frame_count {
                if animation_data.should_loop {
                    self.current_frame_index = 0;
                } else {
                    self.current_frame_index = frame_count - 1;
                    self.can_play = false;
                    break;
                }
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D, pos: Vector2, anim_data: &AnimationData) {
        anim_data.frames[self.current_frame_index as usize].draw(d, pos, texture);
    }

    pub fn reset(&mut self) {
        self.current_frame_index = 0;
        self.current_frame_time = 0.0;
        self.can_play = true;
    }
}
