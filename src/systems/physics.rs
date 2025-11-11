use crate::core::{
    game_zone::{self},
    states::playing::PlayingState,
};

pub fn update(state: &mut PlayingState, dt: f32) {
    let zone_meta = game_zone::get_zone_meta_for_distance(state.distance_traveled);

    if state.distance_traveled > 0.0 {
        state.speed += zone_meta.gravity * dt;
    }

    state.distance_traveled += state.speed * dt;

    for o in &mut state.obstacles {
        o.position += o.direction * o.speed * dt;

        let speed_delta = state.speed - o.rel_speed;

        if speed_delta != 0.0 {
            let mut rel_speed = o.speed + speed_delta;

            rel_speed = if o.rel_sign > 0.0 {
                rel_speed.max(0.0)
            } else {
                rel_speed.min(0.0)
            };

            o.position.y += rel_speed * dt;
        }
    }
}

pub fn get_player_obstacle_collision(state: &PlayingState) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let player_collider = state.player.collider();

    for (idx, o) in state.obstacles.iter().enumerate() {
        if o.collider().check_collision_recs(&player_collider) {
            result.push(idx);
        }
    }

    result
}
