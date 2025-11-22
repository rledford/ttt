use crate::{core::game_state::GameState, systems::input::InputState};

pub struct GameOverState {
    pub time: f32,
}

pub fn update(state: &mut GameOverState, input: &InputState, dt: f32) -> Option<GameState> {
    state.time += dt;

    if input.boost_pressed {
        return Some(GameState::Playing);
    }

    None
}
