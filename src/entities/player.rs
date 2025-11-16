use raylib::prelude::*;

use crate::entities::particle::{BOOST_EMITTER_CONFIG, ParticleEmitter, ParticleEmitterShape};

pub struct Player {
    pub position: Vector2,
    pub is_boosting: bool,
    pub aabb: Rectangle,
    pub left_boost: ParticleEmitter,
    pub right_boost: ParticleEmitter,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        let mut left_boost: ParticleEmitter = ParticleEmitter::new(500);
        left_boost.offset = Vector2::new(-6.0, 16.0);
        left_boost.config = BOOST_EMITTER_CONFIG;
        left_boost.shape = ParticleEmitterShape::Cone {
            direction: Vector2::new(0.0, 1.0),
            angle: 0.77,
        };

        let mut right_boost: ParticleEmitter = ParticleEmitter::new(500);
        right_boost.offset = Vector2::new(6.0, 16.0);
        right_boost.config = BOOST_EMITTER_CONFIG;
        right_boost.shape = ParticleEmitterShape::Cone {
            direction: Vector2::new(0.0, 1.0),
            angle: 0.77,
        };

        Self {
            position: Vector2::new(x, y),
            aabb: Rectangle::new(0.0, 0.0, 24.0, 32.0),
            is_boosting: false,
            left_boost,
            right_boost,
        }
    }
    pub fn collider(&self) -> Rectangle {
        Rectangle {
            x: self.position.x - self.aabb.width * 0.5,
            y: self.position.y - self.aabb.height * 0.5,
            width: self.aabb.width,
            height: self.aabb.height,
        }
    }
    pub fn trigger_boost_burst(&mut self) {
        self.left_boost.burst();
        self.right_boost.burst();
    }
}
