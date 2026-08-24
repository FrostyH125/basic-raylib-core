use crate::{raylib::sprite::Sprite, raylib::sprite_animation::SpriteAnimationInstance};
use raylib::prelude::*;

pub struct SpriteAnimationData {
    pub frames: &'static [Sprite],
    pub frame_duration: f32,
    pub should_loop: bool,
}
