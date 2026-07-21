use rand::{RngExt, rngs::ThreadRng};
use raylib::math::{Rectangle, Vector2};

#[inline]
/// this only works if t starts from 0
pub fn smooth_lerp(start_n: f32, end_n: f32, current_t: f32, total_t: f32) -> f32 {
    let progress = current_t / total_t;
    let t = progress.clamp(0.0, 1.0);
    let weight = t * t * (3.0 - 2.0 * t);
    return start_n + (end_n - start_n) * weight;
}

#[inline]
/// provided a t value between min_t and max_t, will give an n value between start_n and end_n
pub fn smooth_lerp_min_max(start_n: f32, end_n: f32, current_t: f32, start_t: f32, end_t: f32) -> f32 {
    let progress = progress(start_t, end_t, current_t);
    let weight = progress * progress * (3.0 - 2.0 * progress);
    return start_n + (end_n - start_n) * weight;
}

/// returns a value 0.0..=1.0 based on how far along current is to start and end
#[inline]
pub fn progress(start: f32, end: f32, current: f32) -> f32 {
    return ((current - start) / (end - start)).clamp(0.0, 1.0);
}

/// returns a Vector2 with x & y: -1.0..=1.0
/// takes a threadrng as to avoid allocating one through repeated calls
#[inline]
pub fn random_dir(rng: &mut ThreadRng) -> Vector2 {
    let rand_x = rng.random_range(-1.0..=1.0);
    let rand_y = rng.random_range(-1.0..=1.0);

    return Vector2 { x: rand_x, y: rand_y };
}
#[inline]
pub fn center_of_rect(rect: Rectangle) -> Vector2 {
    return Vector2::new(rect.x + (rect.width / 2.0), rect.y + (rect.height / 2.0));
}
