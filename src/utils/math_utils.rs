pub fn smooth_lerp(start: f32, end: f32, current_time: f32, total_duration: f32) -> f32 {
    let progress = current_time / total_duration;
    let t = progress.clamp(0.0, 1.0);
    let weight = t * t * (3.0-2.0 * t);
    start + (end - start) * weight
}