#[derive(Default)]
pub struct SpriteAnimationInstance {
    pub current_frame_time: f32,
    pub current_frame_index: u8,
    pub can_play: bool,
}

impl SpriteAnimationInstance {
    pub fn reset(&mut self) {
        self.current_frame_index = 0;
        self.current_frame_time = 0.0;
        self.can_play = true;
    }
}

