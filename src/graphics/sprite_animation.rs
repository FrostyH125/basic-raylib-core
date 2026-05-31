#[derive(Default)]
pub struct SpriteAnimationInstance {
    pub current_frame_time: f32,
    pub current_frame_index: u8,
    pub finished_playing: bool,
}

impl SpriteAnimationInstance {
    pub fn new() -> Self {
        SpriteAnimationInstance {
            current_frame_time: 0.0,
            current_frame_index: 0,
            finished_playing: false,
        }
    }

    pub fn reset(&mut self) {
        self.current_frame_index = 0;
        self.current_frame_time = 0.0;
        self.finished_playing = true;
    }
}
