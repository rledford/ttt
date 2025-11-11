use crate::core::game_state::GameState;

pub struct LoadingState {
    pub timer: f32,
}

pub fn update(state: &mut LoadingState, dt: f32) -> Option<GameState> {
    state.timer -= dt;

    if state.timer <= 0.0 {
        return Some(GameState::Playing);
    }

    None
}
