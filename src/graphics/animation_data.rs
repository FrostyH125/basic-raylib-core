use crate::{graphics::sprite_animation::SpriteAnimationInstance, graphics::sprite::Sprite};
use raylib::prelude::*;

pub struct AnimationData {
    pub frames: &'static [Sprite],
    pub frame_duration: f32,
    pub should_loop: bool,
}

impl AnimationData {
    pub fn update(&self, animation_instance: &mut SpriteAnimationInstance, dt: f32) {
        let frame_count = self.frames.len();
        
        if !animation_instance.can_play {
            return;
        }
        
        if frame_count == 0 {
            animation_instance.can_play = false;
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
                    animation_instance.can_play = false;
                    break;
                }
            }
        }
    }

        pub fn draw(&self, animation_instance: &SpriteAnimationInstance, d: &mut RaylibDrawHandle, texture: &Texture2D, pos: Vector2) {
            self.frames[animation_instance.current_frame_index as usize].draw(d, pos, texture);
    }
}
