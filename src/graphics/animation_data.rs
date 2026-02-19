use crate::sprite::Sprite;

pub struct AnimationData {
    pub frames: Vec<Sprite>,
    pub frame_duration: f32,
    pub should_loop: bool,
}