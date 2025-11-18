use raylib::prelude::*;

#[derive(Clone, Copy)]
pub enum MovementAxis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy)]
pub enum MovementPattern {
    Static,
    LinearHorizontal,
    LinearVertical,
    LinearDiagnal,
    Sinwave {
        axis: MovementAxis,
        frequency: f32,
        amplitude: f32,
    },
    Circular {
        radius: f32,
        clockwise: bool,
    },
}

#[derive(Clone, Copy)]
pub enum ObstacleType {
    LaunchDebris,
    FlockOfBirds,
    WeatherBallon,
    Drone,
}

impl ObstacleType {
    pub fn aabb(&self) -> Rectangle {
        match self {
            ObstacleType::LaunchDebris => Rectangle::new(0.0, 0.0, 26.0, 20.0),
            ObstacleType::FlockOfBirds => Rectangle::new(0.0, 0.0, 40.0, 24.0),
            ObstacleType::WeatherBallon => Rectangle::new(0.0, 0.0, 50.0, 60.0),
            ObstacleType::Drone => Rectangle::new(0.0, 0.0, 20.0, 20.0),
        }
    }
}

pub struct Obstacle {
    pub kind: ObstacleType,
    pub position: Vector2,
    pub direction: Vector2,
    pub aabb: Rectangle,
    pub speed: f32,
    pub rel_speed: f32, // relative to world speed
    pub rel_sign: f32,  // relative to world direction
    pub is_in_destruction_range: bool,

    pub movement_pattern: MovementPattern,
    pub movement_time: f32,
    pub spawn_position: Vector2,
}

impl Obstacle {
    pub fn collider(&self) -> Rectangle {
        Rectangle {
            x: self.position.x - self.aabb.width * 0.5,
            y: self.position.y - self.aabb.height * 0.5,
            width: self.aabb.width,
            height: self.aabb.height,
        }
    }

    pub fn new(
        kind: ObstacleType,
        position: Vector2,
        direction: Vector2,
        speed: f32,
        rel_speed: f32,
        rel_sign: f32,
        movement_pattern: MovementPattern,
    ) -> Self {
        Self {
            kind,
            position,
            direction,
            spawn_position: position,
            aabb: kind.aabb(),
            speed,
            rel_speed,
            rel_sign,
            is_in_destruction_range: false,
            movement_pattern,
            movement_time: 0.0,
        }
    }
}
