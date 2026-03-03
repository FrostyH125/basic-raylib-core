use raylib::text::{Font, RaylibFont};

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
    }

    if !current_line.is_empty() {
        final_string.push_str(&current_line);
    }

    final_string
}
