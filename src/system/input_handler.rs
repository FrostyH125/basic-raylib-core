use raylib::prelude::{MouseButton::*, *};

pub struct InputState {
    pub mouse_pos: Vector2,
    pub delta: Vector2,
    
    left_click_pos: Vector2,
    pub left_stopped_dragging_this_frame: bool,
    pub left_currently_held: bool,
    pub left_clicked_once: bool,
    pub left_dragging: bool,

    right_click_pos: Vector2,
    pub right_stopped_dragging_this_frame: bool,
    pub right_currently_held: bool,
    pub right_clicked_once: bool,
    pub right_dragging: bool,

    middle_click_pos: Vector2,
    pub middle_stopped_dragging_this_frame: bool,
    pub middle_currently_held: bool,
    pub middle_clicked_once: bool,
    pub middle_dragging: bool,

    pub middle_roll: f32
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            mouse_pos: Default::default(),
            right_click_pos: Default::default(),
            delta: Default::default(),
            right_stopped_dragging_this_frame: false,
            right_currently_held: false,
            right_clicked_once: false,
            right_dragging: false,
            left_click_pos: Default::default(),
            left_stopped_dragging_this_frame: false,
            left_currently_held: false,
            left_clicked_once: false,
            left_dragging: false,
            middle_click_pos: Default::default(),
            middle_stopped_dragging_this_frame: false,
            middle_currently_held: false,
            middle_clicked_once: false,
            middle_dragging: false,
            middle_roll: Default::default()
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, camera_zoom: f32) {
        // get mouse pos
        self.mouse_pos = rl.get_mouse_position() / camera_zoom;

        // handle left mouse button
        self.left_currently_held = rl.is_mouse_button_down(MOUSE_BUTTON_LEFT);
        self.left_clicked_once = rl.is_mouse_button_pressed(MOUSE_BUTTON_LEFT);
            
        if self.left_clicked_once {
            self.left_click_pos = self.mouse_pos;
        }

        let dx = self.mouse_pos.x - self.left_click_pos.x;
        let dy = self.mouse_pos.y - self.left_click_pos.y;

        let distance_between_left_click_and_current_pos_squared = dx * dx + dy * dy;
        self.left_dragging = self.left_currently_held && distance_between_left_click_and_current_pos_squared >= 0.1 * 0.1;

        // handle right mouse button
        self.right_currently_held = rl.is_mouse_button_down(MOUSE_BUTTON_RIGHT);
        self.right_clicked_once = rl.is_mouse_button_pressed(MOUSE_BUTTON_RIGHT);
            
        if self.right_clicked_once {
            self.right_click_pos = self.mouse_pos;
        }

        let dx = self.mouse_pos.x - self.right_click_pos.x;
        let dy = self.mouse_pos.y - self.right_click_pos.y;

        let distance_between_right_click_and_current_pos_squared = dx * dx + dy * dy;
        self.right_dragging = self.right_currently_held && distance_between_right_click_and_current_pos_squared >= 0.1 * 0.1;

        // handle middle mouse button
        self.middle_currently_held = rl.is_mouse_button_down(MOUSE_BUTTON_MIDDLE);
        self.middle_clicked_once = rl.is_mouse_button_pressed(MOUSE_BUTTON_MIDDLE);
            
        if self.middle_clicked_once {
            self.middle_click_pos = self.mouse_pos;
        }

        let dx = self.mouse_pos.x - self.middle_click_pos.x;
        let dy = self.mouse_pos.y - self.middle_click_pos.y;

        let distance_between_click_and_current_pos_squared = dx * dx + dy * dy;
        self.middle_dragging = self.middle_currently_held && distance_between_click_and_current_pos_squared >= 0.1 * 0.1;

        self.delta = rl.get_mouse_delta();
        self.middle_roll = rl.get_mouse_wheel_move();
    }

    pub fn reset_and_set_zero_inputs(&mut self) {
        self.right_stopped_dragging_this_frame = false;
        self.right_clicked_once = false;
        self.right_currently_held = false;
        self.right_dragging = false;

        self.left_stopped_dragging_this_frame = false;
        self.left_clicked_once = false;
        self.left_currently_held = false;
        self.left_dragging = false;

        self.middle_stopped_dragging_this_frame = false;
        self.middle_clicked_once = false;
        self.middle_currently_held = false;
        self.middle_dragging = false;
    }
}