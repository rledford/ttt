use raylib::prelude::*;

use crate::{
    core::game_state::{GameState, V_HEIGHT, V_WIDTH},
    entities::{obstacle::Obstacle, player::Player},
    systems::input::InputState,
};

pub struct PlayingState {
    pub hp: i32,
    pub max_hp: i32,
    pub heat: f32,
    pub heat_per_boost: f32,
    pub heat_decay: f32,
    pub speed: f32,
    pub boost: f32,
    pub boost_bonus: f32,
    pub boost_decay: f32,
    pub destruction_window: f32,
    pub destruction_window_timer: f32,
    pub distance_traveled: f32,

    pub player: Player,
    pub obstacles: Vec<Obstacle>,
}

impl Default for PlayingState {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayingState {
    pub fn new() -> Self {
        Self {
            hp: 5,
            max_hp: 5,
            heat: 0.0,
            heat_per_boost: 10.0,
            heat_decay: 12.0,
            speed: 0.0,
            boost: 0.0,
            boost_bonus: 0.0,
            boost_decay: 0.85,
            destruction_window: 0.5,
            destruction_window_timer: 0.0,
            distance_traveled: 0.0,
            player: Player {
                position: Vector2 {
                    x: (V_WIDTH as f32) * 0.5,
                    y: (V_HEIGHT as f32) * 0.8,
                },
                aabb: Rectangle::new(0.0, 0.0, 24.0, 32.0),
            },
            obstacles: vec![],
        }
    }
}

pub fn update(state: &mut PlayingState, input: &InputState, dt: f32) -> Option<GameState> {
    crate::systems::boost::update(state, input, dt);
    crate::systems::physics::update(state, dt);
    crate::systems::combat::update(state, dt);
    crate::systems::spawn::update(state, dt);

    None
}
