use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::Vector2,
    text::{Font, RaylibFont},
};

use crate::system::timer::Timer;

pub fn wrap_string(string: &str, max_length_in_pixels: f32, font: &Font, font_size: f32, spacing: f32) -> String {
    let words: Vec<&str> = string.split(" ").collect();
    let mut final_string = String::new();
    let mut current_line = String::new();

    for word in words {
        let original_len = current_line.len();

        if !current_line.is_empty() {
            current_line.push(' ');
        }

        current_line.push_str(word);

        let line_size = font.measure_text(&current_line, font_size, spacing);

        if line_size.x >= max_length_in_pixels {
            current_line.truncate(original_len);

            if !current_line.is_empty() {
                final_string.push_str(&current_line);
                final_string.push('\n');
            }

            current_line.clear();
            current_line.push_str(word);
        }

        //only reasonably should happen if the word ends with \n
        if current_line.contains('\n') {
            final_string.push_str(&current_line);
            current_line.clear();
        }
    }

    if !current_line.is_empty() {
        final_string.push_str(&current_line);
    }

    return final_string;
}

pub fn draw_string_centered_on_pos(
    d: &mut RaylibDrawHandle,
    pos: Vector2,
    string: &str,
    font: &Font,
    font_size: f32,
    spacing: f32,
    color: Color,
) {
    let size_of_string = font.measure_text(string, font_size, spacing);

    let draw_pos = pos - (size_of_string / 2.0);

    d.draw_text_ex(font, string, draw_pos, font_size, spacing, color);
}

pub fn draw_string_with_horizontal_sin_wave(
    d: &mut RaylibDrawHandle,
    sin_wave_timer: &Timer,
    pos: Vector2,
    str: &str,
    max_wave_height: f32,
    font: &Font,
    font_size: f32,
    color: Color
) {

    // necessary buffer for a low allocation char -> &str conversion
    let mut ch_buffer = [0u8, 4];

    let progress = sin_wave_timer.progress();
    let amplitude = 1.0 - progress;
    let t = sin_wave_timer.current_time;
    let num_of_cycles = 10.0;

    // makes it so each char is not lined up in the wave
    let mut sin_offset = 0.1;

    let ch_size = font.measure_text(str, font_size, 0.0) / Vector2::new(str.len() as f32, 1.0);

    for (i, ch) in str.char_indices() {
        let x = pos.x + ch_size.x * i as f32;
        let y = pos.y + (t * num_of_cycles + sin_offset).sin() * amplitude * max_wave_height;

        // necessary as d.draw_text_ex cannot take a char
        // needs to manually encode the character into a 4 byte buffer 
        // and get a reference to it as a &str
        let s = ch.encode_utf8(&mut ch_buffer);

        d.draw_text_ex(font, s, Vector2::new(x, y), font_size, 0.0, color);

        sin_offset += 0.1;
    }
}
