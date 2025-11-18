use crate::core::states::playing::PlayingState;

pub fn update(state: &mut PlayingState, dt: f32) {
    for o in &mut state.obstacles {
        o.position += o.direction * o.speed * dt;
    }
}
