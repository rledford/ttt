use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
};

use crate::{
    core::{
        game_state::V_WIDTH,
        game_zone,
        spawn_config::{self, ObstacleSpawnParams, ZoneSpawnConfig},
        states::playing::PlayingState,
    },
    entities::obstacle::{self},
};

pub fn update(state: &mut PlayingState, dt: f32) {
    let current_zone = game_zone::get_zone_for_distance(state.distance_traveled);
    let spawn_config = spawn_config::get_zone_spawn_config(current_zone);

    let mut rng = rand::rng();
    let v: f32 = rng.random();

    if v < spawn_config.spawn_rate * state.speed * dt {
        let params = pick_obstacle_from_zone_spawn_config(spawn_config);

        let mut o = obstacle::Obstacle::new(params.kind);
        let mut rng = rand::rng();

        o.position.x = rng.random_range(0..V_WIDTH) as f32;
        o.direction.y = 1.0;
        o.speed = rng.random_range(params.speed_range.0..=params.speed_range.1);
        o.rel_speed = state.speed;
        o.rel_sign = 1.0;

        state.obstacles.push(o)
    }
}

fn pick_obstacle_from_zone_spawn_config(config: &ZoneSpawnConfig) -> &ObstacleSpawnParams {
    let weights: Vec<u32> = config.obstacle_pool.iter().map(|p| p.weight).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::rng();
    let idx = dist.sample(&mut rng);

    &config.obstacle_pool[idx]
}
