use raylib::prelude::*;

pub struct InputState {
    pub debug_switch_playing: bool,
    pub debug_switch_depot: bool,
    pub boost_held: bool,
    pub boost_pressed: bool,
    pub boost_released: bool,
    pub ui_left_pressed: bool,
    pub ui_right_pressed: bool,
    pub ui_select_pressed: bool,
    pub ui_cursor_position: Vector2,
}

impl Default for InputState {
    fn default() -> Self {
        InputState::new()
    }
}

impl InputState {
    pub fn new() -> Self {
        Self {
            debug_switch_playing: false,
            debug_switch_depot: false,
            boost_held: false,
            boost_pressed: false,
            boost_released: false,
            ui_left_pressed: false,
            ui_right_pressed: false,
            ui_select_pressed: false,
            ui_cursor_position: Vector2::zero(),
        }
    }

    pub fn ui_horizontal_axis(&self) -> isize {
        let left = if self.ui_left_pressed { -1 } else { 0 };
        let right = if self.ui_right_pressed { 1 } else { 0 };

        left + right
    }
}

pub fn read_input(rl: &RaylibHandle) -> InputState {
    let mut input = InputState::new();

    input.boost_held = rl.is_key_down(KeyboardKey::KEY_SPACE);
    input.boost_pressed = rl.is_key_pressed(KeyboardKey::KEY_SPACE);
    input.boost_released = rl.is_key_released(KeyboardKey::KEY_SPACE);

    input.ui_left_pressed = rl.is_key_pressed(KeyboardKey::KEY_LEFT);
    input.ui_right_pressed = rl.is_key_pressed(KeyboardKey::KEY_RIGHT);
    input.ui_select_pressed = rl.is_key_pressed(KeyboardKey::KEY_SPACE)
        || rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT);

    input.ui_cursor_position = rl.get_mouse_position();

    input.debug_switch_playing =
        rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) && rl.is_key_pressed(KeyboardKey::KEY_P);

    input.debug_switch_depot =
        rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) && rl.is_key_pressed(KeyboardKey::KEY_D);

    input
}
