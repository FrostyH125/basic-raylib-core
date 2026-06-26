use rand::{RngExt, rngs::ThreadRng};
use raylib::math::Vector2;

pub fn smooth_lerp(start: f32, end: f32, current_time: f32, total_duration: f32) -> f32 {
    let progress = current_time / total_duration;
    let t = progress.clamp(0.0, 1.0);
    let weight = t * t * (3.0-2.0 * t);
    start + (end - start) * weight
}

/// returns a Vector2 with x & y: -1.0..=1.0
/// takes a threadrng as to avoid allocating one through repeated calls
pub fn random_dir(rng: &mut ThreadRng) -> Vector2 {
    let rand_x = rng.random_range(-1.0..=1.0);
    let rand_y = rng.random_range(-1.0..=1.0);
    
    return Vector2 { x: rand_x, y: rand_y };
}