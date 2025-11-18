use raylib::prelude::*;

use crate::core::{
    game_zone::{self},
    states::playing::PlayingState,
};

pub fn update(state: &mut PlayingState, dt: f32) {
    let zone_meta = game_zone::get_zone_meta_for_distance(state.distance_traveled);
    let player_collider = state.player.collider();

    if state.distance_traveled > 0.0 {
        state.speed += zone_meta.gravity * dt;
    }

    state.distance_traveled += state.speed * dt;

    for o in &mut state.obstacles {
        let world_drift = (state.speed - o.rel_speed) * dt;

        o.position.y += world_drift;

        let closest_face_dist = get_closest_face_distance(&o.collider(), &player_collider);

        o.is_in_destruction_range =
            closest_face_dist > 0.0 && closest_face_dist <= state.destruction_range
    }
}

pub fn get_player_obstacle_collisions(state: &PlayingState) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let player_collider = state.player.collider();

    for (idx, o) in state.obstacles.iter().enumerate() {
        if o.collider().check_collision_recs(&player_collider) {
            result.push(idx);
        }
    }

    result
}

fn get_closest_face_distance(rect_a: &Rectangle, rect_b: &Rectangle) -> f32 {
    let left_a = rect_a.x;
    let right_a = rect_a.x + rect_a.width;
    let top_a = rect_a.y;
    let bottom_a = rect_a.y + rect_a.height;

    let left_b = rect_b.x;
    let right_b = rect_b.x + rect_b.width;
    let top_b = rect_b.y;
    let bottom_b = rect_b.y + rect_b.height;

    let x_gap = if right_a < left_b {
        left_b - right_a
    } else if right_b < left_a {
        left_a - right_b
    } else {
        0.0
    };

    let y_gap = if bottom_a < top_b {
        top_b - bottom_a
    } else if bottom_b < top_a {
        top_a - bottom_b
    } else {
        0.0
    };

    if x_gap == 0.0 && y_gap == 0.0 {
        0.0
    } else if x_gap > 0.0 && y_gap > 0.0 {
        (x_gap * x_gap + y_gap * y_gap).sqrt()
    } else {
        x_gap.max(y_gap)
    }
}
