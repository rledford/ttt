use crate::{
    core::{
        game_state::{GameState, V_HEIGHT, V_WIDTH},
        perk::{Perk, PerkData, StatModifiers},
    },
    entities::{obstacle::Obstacle, player::Player},
    systems::input::InputState,
};

pub struct PlayingState {
    pub hp: i32,
    pub max_hp: i32,
    pub heat: f32,
    pub speed: f32,
    pub boost: f32,
    pub boost_bonus: f32,
    pub last_boost_activation_time: f64,
    pub destruction_range: f32,
    pub destruction_window: f32,
    pub destruction_window_timer: f32,
    pub distance_traveled: f32,

    pub player: Player,
    pub obstacles: Vec<Obstacle>,

    pub perks: Vec<Perk>,
    pub stat_mods: StatModifiers,
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
            speed: 0.0,
            boost: 0.0,
            boost_bonus: 0.0,
            last_boost_activation_time: 0.0,
            destruction_range: 64.0,
            destruction_window: 0.5,
            destruction_window_timer: 0.0,
            distance_traveled: 0.0,
            player: Player::new((V_WIDTH as f32) * 0.5, (V_HEIGHT as f32) * 0.8),

            obstacles: vec![],
            perks: vec![],
            stat_mods: StatModifiers::default(),
        }
    }

    pub fn try_consume_shield(&mut self) -> bool {
        for p in &mut self.perks {
            match &mut p.data {
                PerkData::Shield { charges, .. } => {
                    if *charges > 0 {
                        *charges -= 1;
                        println!("BLOCKED!");
                        return true;
                    }
                }
            }
        }

        false
    }
}

pub fn update(state: &mut PlayingState, input: &InputState, dt: f32, gt: f64) -> Option<GameState> {
    crate::systems::perk::update(state, dt);
    crate::systems::boost::update(state, input, dt, gt);
    crate::systems::movement::update(state, dt);
    crate::systems::physics::update(state, dt);
    crate::systems::combat::update(state, dt, gt);
    crate::systems::spawn::update(state, dt);
    crate::systems::particle::update(state, dt);

    if state.hp <= 0 {
        return Some(GameState::GameOver);
    }

    None
}
