use crate::{core::game_zone::GameZone, entities::obstacle::ObstacleType};

pub struct ObstacleSpawnParams {
    pub kind: ObstacleType,
    pub weight: u32,
    pub speed_range: (f32, f32),
}

pub struct ZoneSpawnConfig {
    pub spawn_rate: f32,
    pub obstacle_pool: &'static [ObstacleSpawnParams],
}

pub fn get_zone_spawn_config(zone: GameZone) -> &'static ZoneSpawnConfig {
    match zone {
        GameZone::Ground => &GROUND_ZONE_SPAWN_CONFIG,
        GameZone::Troposphere => &TROPOSPHERE_ZONE_SPAWN_CONFIG,
        GameZone::Stratosphere => &STRATOSPHERE_ZONE_SPAWN_CONFIG,
        GameZone::LowOrbit => &LOW_ORBIT_ZONE_SPAWN_CONFIG,
        GameZone::HighOrbit => &HIGH_ORBIT_ZONE_SPAWN_CONFIG,
        GameZone::DeepSpace => &DEEP_SPACE_ZONE_SPAWN_CONFIG,
    }
}

static GROUND_ZONE_SPAWN_CONFIG: ZoneSpawnConfig = ZoneSpawnConfig {
    spawn_rate: 0.015,
    obstacle_pool: &[
        ObstacleSpawnParams {
            kind: ObstacleType::LaunchDebris,
            weight: 50,
            speed_range: (50.0, 100.0),
        },
        ObstacleSpawnParams {
            kind: ObstacleType::FlockOfBirds,
            weight: 30,
            speed_range: (50.0, 100.0),
        },
    ],
};

static TROPOSPHERE_ZONE_SPAWN_CONFIG: ZoneSpawnConfig = ZoneSpawnConfig {
    spawn_rate: 0.018,
    obstacle_pool: &[ObstacleSpawnParams {
        kind: ObstacleType::LaunchDebris,
        weight: 50,
        speed_range: (50.0, 100.0),
    }],
};

static STRATOSPHERE_ZONE_SPAWN_CONFIG: ZoneSpawnConfig = ZoneSpawnConfig {
    spawn_rate: 0.020,
    obstacle_pool: &[ObstacleSpawnParams {
        kind: ObstacleType::LaunchDebris,
        weight: 50,
        speed_range: (50.0, 100.0),
    }],
};

static LOW_ORBIT_ZONE_SPAWN_CONFIG: ZoneSpawnConfig = ZoneSpawnConfig {
    spawn_rate: 0.022,
    obstacle_pool: &[ObstacleSpawnParams {
        kind: ObstacleType::LaunchDebris,
        weight: 50,
        speed_range: (50.0, 100.0),
    }],
};

static HIGH_ORBIT_ZONE_SPAWN_CONFIG: ZoneSpawnConfig = ZoneSpawnConfig {
    spawn_rate: 0.024,
    obstacle_pool: &[ObstacleSpawnParams {
        kind: ObstacleType::LaunchDebris,
        weight: 50,
        speed_range: (50.0, 100.0),
    }],
};

static DEEP_SPACE_ZONE_SPAWN_CONFIG: ZoneSpawnConfig = ZoneSpawnConfig {
    spawn_rate: 0.015,
    obstacle_pool: &[ObstacleSpawnParams {
        kind: ObstacleType::LaunchDebris,
        weight: 50,
        speed_range: (50.0, 100.0),
    }],
};
