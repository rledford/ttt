use std::{f32::consts::TAU, iter};

use rand::Rng;
use raylib::prelude::*;

pub struct Particle {
    pub position: Vector2,
    pub velocity: Vector2,
    pub start_scale: f32,
    pub end_scale: f32,
    pub lifetime: f32,
    pub age: f32,
    pub color: Color,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            position: Vector2::zero(),
            velocity: Vector2::zero(),
            start_scale: 1.0,
            end_scale: 1.0,
            age: 0.0,
            lifetime: 0.0,
            color: Color::BLANK,
        }
    }
}

impl Particle {
    pub fn is_alive(&self) -> bool {
        self.age < self.lifetime
    }
}

#[derive(Default, Clone, Copy)]
pub enum ParticleEmitterShape {
    #[default]
    Point,
    Box {
        width: f32,
        height: f32,
    },
    Circle {
        radius: f32,
    },
    Cone {
        direction: Vector2,
        angle: f32,
    },
    Line {
        offset: Vector2,
    },
}

#[derive(Default, Clone, Copy)]
pub struct ParticleEmitterConfig {
    pub emission_rate: f32,
    pub burst_count: usize,
    pub min_speed: f32,
    pub max_speed: f32,
    pub min_lifetime: f32,
    pub max_lifetime: f32,
    pub start_scale: f32,
    pub end_scale: f32,
    pub color: Color,
}

#[derive(Default)]
pub struct ParticleEmitter {
    pub shape: ParticleEmitterShape,
    pub config: ParticleEmitterConfig,
    pub position: Vector2,
    pub offset: Vector2,
    pub particles: Vec<Particle>,
    pub max_particles: usize,
    pub active_particle_count: usize,
    pub spawn_accumulator: f32,
    pub is_enabled: bool,
}

impl ParticleEmitter {
    pub fn new(max_particles: usize) -> Self {
        Self {
            shape: ParticleEmitterShape::default(),
            config: ParticleEmitterConfig::default(),
            position: Vector2::zero(),
            offset: Vector2::zero(),
            particles: iter::repeat_with(Particle::default)
                .take(max_particles)
                .collect(),
            max_particles,
            active_particle_count: 0,
            spawn_accumulator: 0.0,
            is_enabled: false,
        }
    }
    pub fn active_particles(&self) -> &[Particle] {
        &self.particles[..self.active_particle_count]
    }
    pub fn active_particles_mut(&mut self) -> &mut [Particle] {
        &mut self.particles[..self.active_particle_count]
    }
    pub fn update(&mut self, dt: f32) {
        let mut i = 0;

        while i < self.active_particle_count {
            let particle = &mut self.particles[i];
            particle.age += dt;
            if !particle.is_alive() {
                self.active_particle_count -= 1;
                self.particles.swap(i, self.active_particle_count);
            } else {
                particle.position += particle.velocity * dt;
                i += 1;
            }
        }

        if !self.is_enabled {
            return;
        }

        self.spawn_accumulator += self.config.emission_rate * dt;

        while self.spawn_accumulator >= 1.0 {
            self.spawn_particle();
            self.spawn_accumulator -= 1.0;
        }
    }

    pub fn spawn_particle(&mut self) {
        if self.active_particle_count == self.max_particles {
            return;
        }

        let mut rng = rand::rng();
        let particle = &mut self.particles[self.active_particle_count];

        particle.age = 0.0;
        particle.start_scale = self.config.start_scale;
        particle.end_scale = self.config.end_scale;
        particle.lifetime = rng.random_range(self.config.min_lifetime..self.config.max_lifetime);
        particle.color = self.config.color;

        particle.position = get_particle_position(self.shape, self.position, &mut rng);
        particle.velocity = get_particle_velocity(
            self.shape,
            self.position,
            particle.position,
            self.config.min_speed,
            self.config.max_speed,
            &mut rng,
        );

        self.active_particle_count += 1;
    }

    pub fn burst(&mut self) {
        let mut i = self.config.burst_count;
        while i > 0 {
            self.spawn_particle();
            i -= 1;
        }
    }
}

pub fn get_particle_position(
    emitter_shape: ParticleEmitterShape,
    emitter_pos: Vector2,
    rng: &mut impl Rng,
) -> Vector2 {
    match emitter_shape {
        ParticleEmitterShape::Box { width, height } => {
            emitter_pos
                + Vector2::new(
                    rng.random_range(-width * 0.5..width * 0.5),
                    rng.random_range(-height * 0.5..height * 0.5),
                )
        }
        ParticleEmitterShape::Circle { radius } => {
            let angle = rng.random_range(0.0..TAU);
            let r = rng.random_range(0.0..radius);

            emitter_pos + Vector2::new(r * angle.cos(), r * angle.sin())
        }
        ParticleEmitterShape::Line { offset } => {
            let slope = offset - emitter_pos;

            emitter_pos + slope * rng.random_range(0.0..1.0)
        }
        _ => emitter_pos,
    }
}

pub fn get_particle_velocity(
    emitter_shape: ParticleEmitterShape,
    emitter_pos: Vector2,
    spawn_pos: Vector2,
    min_speed: f32,
    max_speed: f32,
    rng: &mut impl Rng,
) -> Vector2 {
    match emitter_shape {
        ParticleEmitterShape::Point => {
            Vector2::new(rng.random_range(0.0..1.0), rng.random_range(0.0..1.0)).normalized()
                * rng.random_range(min_speed..max_speed)
        }
        ParticleEmitterShape::Cone { direction, angle } => {
            return direction.rotated(rng.random_range(-angle * 0.5..angle * 0.5))
                * rng.random_range(min_speed..max_speed);
        }
        _ => (spawn_pos - emitter_pos).normalized() * rng.random_range(min_speed..max_speed),
    }
}

pub const BOOST_EMITTER_CONFIG: ParticleEmitterConfig = ParticleEmitterConfig {
    emission_rate: 5.0,
    burst_count: 50,
    min_speed: 5.0,
    max_speed: 25.0,
    min_lifetime: 0.75,
    max_lifetime: 1.5,
    start_scale: 3.0,
    end_scale: 1.0,
    color: Color::GREEN,
};
