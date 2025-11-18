use raylib::prelude::*;

use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
};

use crate::{
    core::{
        game_state::{V_HEIGHT, V_WIDTH},
        game_zone,
        spawn_config::{self, ObstacleSpawnParams, ZoneSpawnConfig},
        states::playing::PlayingState,
    },
    entities::obstacle::{MovementPattern, Obstacle},
};

pub fn update(state: &mut PlayingState, dt: f32) {
    let current_zone = game_zone::get_zone_for_distance(state.distance_traveled);
    let spawn_config = spawn_config::get_zone_spawn_config(current_zone);

    let mut rng = rand::rng();
    let v: f32 = rng.random();

    if v < spawn_config.spawn_rate * state.speed * dt {
        let mut rng = rand::rng();

        let params = pick_obstacle_params(spawn_config);
        let rel_speed = state.speed;
        let rel_sign = 1.0;
        let speed = rng.random_range(params.speed_range.0..=params.speed_range.1);

        let mut position = Vector2::new(rng.random_range(0..V_WIDTH) as f32, 0.0);
        let mut direction = Vector2::zero();

        match params.movement_pattern {
            MovementPattern::LinearHorizontal => {
                if rng.random::<bool>() {
                    position.x = 0.0;
                } else {
                    position.x = V_WIDTH as f32;
                }

                position.y = rng.random_range((V_HEIGHT as f32 * 0.25)..(V_HEIGHT as f32 * 0.5));

                direction.x = (state.player.position - position).x.signum();
                direction.y = -0.5;
            }
            MovementPattern::LinearVertical => {
                direction.y = (state.player.position - position).y.signum();
            }
            MovementPattern::LinearDiagnal => {
                let target =
                    state.player.position - Vector2::new(0.0, rng.random_range(50.0..100.0));

                direction = (target - position).normalized();
            }
            _ => {
                direction.y = 1.0;
            }
        }

        state.obstacles.push(Obstacle::new(
            params.kind,
            position,
            direction,
            speed,
            rel_speed,
            rel_sign,
            params.movement_pattern,
        ))
    }
}

fn pick_obstacle_params(config: &ZoneSpawnConfig) -> &ObstacleSpawnParams {
    let mut rng = rand::rng();

    let weights: Vec<u32> = config.obstacle_pool.iter().map(|p| p.weight).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let idx = dist.sample(&mut rng);

    &config.obstacle_pool[idx]
}
