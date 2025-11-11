use crate::core::states::playing::PlayingState;

pub fn update(state: &mut PlayingState, dt: f32) {
    state.destruction_window_timer = (state.destruction_window_timer - dt).max(0.0);

    let mut collisions = crate::systems::physics::get_player_obstacle_collision(state);
    collisions.sort();
    collisions.reverse();

    for idx in collisions {
        if idx < state.obstacles.len() {
            if state.destruction_window_timer > 0.0 {
                println!("DESTRUCTION!");
                state.heat = (state.heat - 10.0).max(0.0);

                // accumulating destruction_window_timer should be a perk
                // state.destruction_window_timer += state.destruction_window;
                state.destruction_window_timer = 1.5; // state.destruction_window;
                state.boost_bonus += 3.0; // should be 50% of zone cap
            } else {
                println!("Damage taken");
                state.hp -= 1;
            }

            state.obstacles.remove(idx);
        }
    }
}
