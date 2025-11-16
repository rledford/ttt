use crate::{
    core::{
        game_zone::{self},
        states::playing::PlayingState,
    },
    systems::input::InputState,
};

const ZONE_BOOST_CAP_ACCUMULATION_MULTIPLIER: f32 = 0.10;
const BOOST_ACCUMULATION_RATE: f32 = 15.0;

const HEAT_ACCUMULATION_RATE: f32 = 12.0;
const HEAT_DISSIPATION_RATE: f32 = 8.0;

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
        state.boost = (state.boost
            + (BOOST_ACCUMULATION_RATE
                + zone_meta.boost_cap * ZONE_BOOST_CAP_ACCUMULATION_MULTIPLIER)
                * state.stat_mods.boost_multiplier
                * dt)
            .clamp(0.0, zone_meta.boost_cap);

        state.heat = (state.heat + HEAT_ACCUMULATION_RATE * state.stat_mods.heat_multiplier * dt)
            .clamp(0.0, 100.0);

        state.speed += (state.boost + state.boost_bonus) * dt;
    } else if state.boost > 0.0 {
        state.heat = (state.heat - HEAT_DISSIPATION_RATE * dt).clamp(0.0, 100.0);
        state.boost *= state.stat_mods.boost_decay.powf(dt);

        if state.boost <= 0.05 {
            state.boost = 0.0;
        }
    }

    state.player.is_boosting = input.boost_held;
}
