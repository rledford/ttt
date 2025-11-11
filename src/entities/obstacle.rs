use raylib::prelude::*;

#[derive(Clone, Copy)]
pub enum ObstacleType {
    LaunchDebris,
    FlockOfBirds,
    WeatherBallon,
    Drone,
}

pub struct Obstacle {
    pub kind: ObstacleType,
    pub position: Vector2,
    pub direction: Vector2,
    pub aabb: Rectangle,
    pub speed: f32,
    pub rel_speed: f32, // relative to world speed
    pub rel_sign: f32,  // relative to world direction
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

    pub fn new(kind: ObstacleType) -> Self {
        let position = Vector2::zero();
        let direction = Vector2::zero();

        match kind {
            ObstacleType::LaunchDebris => Obstacle {
                kind,
                position,
                direction,
                aabb: Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 26.0,
                    height: 20.0,
                },
                speed: 0.0,
                rel_speed: 0.0,
                rel_sign: 1.0,
            },
            ObstacleType::FlockOfBirds => Obstacle {
                kind,
                position,
                direction,
                aabb: Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 40.0,
                    height: 24.0,
                },
                speed: 0.0,
                rel_speed: 0.0,
                rel_sign: 1.0,
            },
            ObstacleType::WeatherBallon => Obstacle {
                kind,
                position,
                direction,
                aabb: Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 50.0,
                    height: 60.0,
                },
                speed: 0.0,
                rel_speed: 0.0,
                rel_sign: 1.0,
            },

            ObstacleType::Drone => Obstacle {
                kind,
                position,
                direction,
                aabb: Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 20.0,
                    height: 20.0,
                },
                speed: 0.0,
                rel_speed: 0.0,
                rel_sign: 1.0,
            },
        }
    }
}
