use crate::core::{perk::PerkData, states::playing::PlayingState};

pub fn update(state: &mut PlayingState, dt: f32) {
    for p in &mut state.perks {
        match &mut p.data {
            PerkData::Shield {
                max_charges,
                charges,
                recharge_time,
                recharge_timer,
            } => {
                if *charges >= *max_charges {
                    return;
                }

                *recharge_timer -= dt;

                if *recharge_timer <= 0.0 {
                    println!("Shield Recharged");
                    *recharge_timer = *recharge_time;
                    *charges += 1;
                }
            }
        }
    }
}
