use raylib::prelude::*;

pub struct InputState {
    pub boost_held: bool,
    pub boost_pressed: bool,
    pub boost_released: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState::new()
    }
}

impl InputState {
    pub fn new() -> Self {
        Self {
            boost_held: false,
            boost_pressed: false,
            boost_released: false,
        }
    }
}

pub fn read_input(rl: &RaylibHandle) -> InputState {
    let mut input = InputState::new();

    input.boost_held = rl.is_key_down(KeyboardKey::KEY_SPACE);
    input.boost_pressed = rl.is_key_pressed(KeyboardKey::KEY_SPACE);
    input.boost_released = rl.is_key_released(KeyboardKey::KEY_SPACE);

    input
}
