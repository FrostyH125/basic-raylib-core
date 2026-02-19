use raylib::prelude::*;

use crate::graphics::animation_data::AnimationData;

pub struct SpriteAnimationInstance {
    current_frame_time: f32,
    current_frame_index: u8,
    pub is_playing: bool,
}

impl SpriteAnimationInstance {
    pub fn new() -> Self {
        SpriteAnimationInstance {
            current_frame_time: 0.0,
            current_frame_index: 0,
            is_playing: false,
        }
    }

    pub fn update(&mut self, animation_data: &AnimationData, dt: f32) {
        let frame_count = animation_data.frames.len() as u8;

        if frame_count == 0 {
            self.is_playing = false;
            return;
        }

        self.current_frame_time += dt;

        if !animation_data.should_loop {
            if (self.current_frame_index >= frame_count)
                || (frame_count == 1 && self.current_frame_time >= animation_data.frame_duration)
            {
                self.current_frame_index = frame_count - 1;
                self.is_playing = false;
                return;
            }
        }
        
        self.is_playing = true;
        
        if self.current_frame_time >= animation_data.frame_duration {
            self.current_frame_index += 1;
            self.current_frame_time -= animation_data.frame_duration;
            
            if self.current_frame_index >= frame_count {
                if animation_data.should_loop {
                    self.current_frame_index = 0;
                } else {
                    self.current_frame_index = frame_count - 1;
                }
            }
        }
    }
    
    pub fn draw(&self, texture: &Texture2D, pos: Vector2, anim_data: &AnimationData, d: &mut RaylibDrawHandle) {
        anim_data.frames[self.current_frame_index as usize].draw(pos, texture, d);
    }
    
    pub fn reset(&mut self) {
        self.current_frame_index = 0;
        self.current_frame_time = 0.0;
        self.is_playing = false;
    }
}
