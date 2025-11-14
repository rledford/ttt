use crate::{
    core::{
        game_zone::{self},
        states::playing::PlayingState,
    },
    systems::input::InputState,
};

pub fn update(state: &mut PlayingState, input: &InputState, dt: f32, gt: f64) {
    let zone_meta = game_zone::get_zone_meta_for_distance(state.distance_traveled);

    if input.boost_pressed {
        state.destruction_window_timer = state.destruction_window;
    }

    if input.boost_released {
        state.boost_bonus = 0.0;
        state.destruction_window_timer = 0.0;
        state.last_boost_activation_time = gt;
    }

    if input.boost_held {
        state.boost = (state.boost + zone_meta.boost_cap * 0.15).min(zone_meta.boost_cap);
        state.heat = (state.heat + state.heat_per_boost * dt).clamp(0.0, 100.0);

        state.speed += (state.boost + state.boost_bonus) * dt;
    } else if state.boost > 0.0 {
        state.heat = (state.heat - state.heat_decay * dt).clamp(0.0, 100.0);
        state.boost *= state.boost_decay.powf(dt);

        if state.boost <= 0.05 {
            state.boost = 0.0;
        }
    }
}
