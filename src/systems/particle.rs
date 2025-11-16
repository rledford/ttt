use crate::core::states::playing::PlayingState;

pub fn update(state: &mut PlayingState, dt: f32) {
    state.player.left_boost.is_enabled = state.player.is_boosting;
    state.player.left_boost.position = state.player.position + state.player.left_boost.offset;
    state.player.left_boost.update(dt);

    state.player.right_boost.is_enabled = state.player.is_boosting;
    state.player.right_boost.position = state.player.position + state.player.right_boost.offset;
    state.player.right_boost.update(dt);
}
