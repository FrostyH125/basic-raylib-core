use crate::{system::timer::Timer, utils::math_utils::smooth_lerp};

pub struct Tween {
    timer: Timer,
    current_value: f32,
    start_value: f32,
    end_value: f32
}

impl Tween {
    pub fn new(duration: f32, start_value: f32, end_value: f32) -> Self {
        let timer = Timer::new(duration);
        return Self {
            timer,
            current_value: start_value,
            start_value,
            end_value,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.timer.is_done() {
            return;
        }
        self.timer.track(dt);
        self.current_value = smooth_lerp(self.start_value, self.end_value, self.timer.progress());
    }

    pub fn reset(&mut self) {
        self.current_value = self.start_value;
        self.timer.reset();
    }

    pub fn value(&self) -> f32 {
        return self.current_value;
    }
}