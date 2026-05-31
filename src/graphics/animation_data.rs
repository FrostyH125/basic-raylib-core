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

        pub fn draw(&self, animation_instance: &SpriteAnimationInstance, d: &mut RaylibDrawHandle, pos: Vector2, texture: &Texture2D)  {
            self.frames[animation_instance.current_frame_index as usize].draw(d, pos, texture);
    }
}
