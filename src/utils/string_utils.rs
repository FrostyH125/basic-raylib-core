use raylib::{color::Color, drawing::{RaylibDraw, RaylibDrawHandle}, math::Vector2, text::{Font, RaylibFont}};

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

    final_string
}

pub fn draw_string_centered_on_pos(d: &mut RaylibDrawHandle, pos: Vector2, string: &str, font: &Font, font_size: f32, spacing: f32, color: Color) {
    let size_of_string = font.measure_text(string, font_size, spacing);

    let draw_pos = pos - (size_of_string / 2.0);

    d.draw_text_ex(font, string, draw_pos, font_size, spacing, color);
}
