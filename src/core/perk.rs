pub enum PerkType {
    Shield,
}

pub struct Perk {
    pub kind: PerkType,
    pub duration: Option<f32>,
    pub stacks: u32,
    pub data: PerkData,
}

pub enum PerkData {
    Shield {
        max_charges: u32,
        charges: u32,
        recharge_time: f32,
        recharge_timer: f32,
    },
}

pub struct StatModifiers {
    pub heat_multiplier: f32,
    pub boost_strength_multiplier: f32,
    pub boost_decay_rate: f32,
    pub gravity_multiplier: f32,
    pub destruction_window_multiplier: f32,
    pub max_hp_bonus: u32,
}

impl Default for StatModifiers {
    fn default() -> Self {
        Self {
            heat_multiplier: 1.0,
            boost_strength_multiplier: 1.0,
            boost_decay_rate: 0.85,
            gravity_multiplier: 1.0,
            destruction_window_multiplier: 1.0,
            max_hp_bonus: 0,
        }
    }
}
