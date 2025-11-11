const GROUND_MAX: f32 = 6_000.0;
const TROPOSPHERE_MAX: f32 = 11_000.0;
const STRATOSPHERE_MAX: f32 = 15_000.0;
const LOW_ORBIT_MAX: f32 = 20_000.0;
const HIGH_ORBIT_MAX: f32 = 28_000.0;

pub enum GameZone {
    Ground,
    Troposphere,
    Stratosphere,
    LowOrbit,
    HighOrbit,
    DeepSpace,
}

pub struct GameZoneMeta {
    pub name: &'static str,
    pub description: &'static str,
    pub min_dist: f32,
    pub max_dist: Option<f32>,
    pub boost_cap: f32,
    pub gravity: f32,
    pub terminal_velocity: f32,
}

pub fn get_zone_for_distance(dist: f32) -> GameZone {
    match dist {
        ..GROUND_MAX => GameZone::Ground,
        GROUND_MAX..TROPOSPHERE_MAX => GameZone::Troposphere,
        TROPOSPHERE_MAX..STRATOSPHERE_MAX => GameZone::Stratosphere,
        STRATOSPHERE_MAX..LOW_ORBIT_MAX => GameZone::LowOrbit,
        LOW_ORBIT_MAX..HIGH_ORBIT_MAX => GameZone::HighOrbit,
        _ => GameZone::DeepSpace,
    }
}

pub fn get_zone_meta_for_distance(dist: f32) -> &'static GameZoneMeta {
    match dist {
        ..GROUND_MAX => &GROUND_ZONE_META,
        GROUND_MAX..TROPOSPHERE_MAX => &TROPOSPHERE_ZONE_META,
        TROPOSPHERE_MAX..STRATOSPHERE_MAX => &STRATOSPHERE_ZONE_META,
        STRATOSPHERE_MAX..LOW_ORBIT_MAX => &LOW_ORBIT_ZONE_META,
        LOW_ORBIT_MAX..HIGH_ORBIT_MAX => &HIGH_ORBIT_ZONE_META,
        _ => &DEEP_SPACE_ZONE_META,
    }
}

pub fn get_next_zone(current: GameZone) -> Option<GameZone> {
    match current {
        GameZone::Ground => Some(GameZone::Troposphere),
        GameZone::Troposphere => Some(GameZone::Stratosphere),
        GameZone::Stratosphere => Some(GameZone::LowOrbit),
        GameZone::LowOrbit => Some(GameZone::HighOrbit),
        GameZone::HighOrbit => Some(GameZone::DeepSpace),
        _ => None,
    }
}

static GROUND_ZONE_META: GameZoneMeta = GameZoneMeta {
    name: "Ground",
    description: "",
    min_dist: 0.0,
    max_dist: Some(GROUND_MAX),
    boost_cap: 3.0,
    gravity: -0.5,
    terminal_velocity: -15.0,
};

static TROPOSPHERE_ZONE_META: GameZoneMeta = GameZoneMeta {
    name: "Troposphere",
    description: "",
    min_dist: GROUND_MAX,
    max_dist: Some(TROPOSPHERE_MAX),
    boost_cap: 3.5,
    gravity: -1.0,
    terminal_velocity: -12.0,
};

static STRATOSPHERE_ZONE_META: GameZoneMeta = GameZoneMeta {
    name: "Stratosphere",
    description: "",
    min_dist: TROPOSPHERE_MAX,
    max_dist: Some(STRATOSPHERE_MAX),
    boost_cap: 4.0,
    gravity: -0.8,
    terminal_velocity: -10.0,
};

static LOW_ORBIT_ZONE_META: GameZoneMeta = GameZoneMeta {
    name: "Low Orbit",
    description: "",
    min_dist: STRATOSPHERE_MAX,
    max_dist: Some(LOW_ORBIT_MAX),
    boost_cap: 4.5,
    gravity: -0.5,
    terminal_velocity: -5.0,
};

static HIGH_ORBIT_ZONE_META: GameZoneMeta = GameZoneMeta {
    name: "High Orbit",
    description: "",
    min_dist: LOW_ORBIT_MAX,
    max_dist: Some(HIGH_ORBIT_MAX),
    boost_cap: 5.0,
    gravity: -0.2,
    terminal_velocity: -2.0,
};

static DEEP_SPACE_ZONE_META: GameZoneMeta = GameZoneMeta {
    name: "Deep Space",
    description: "",
    min_dist: HIGH_ORBIT_MAX,
    max_dist: None,
    boost_cap: 5.5,
    gravity: 0.0,
    terminal_velocity: 0.0,
};
