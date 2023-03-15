use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Wanderer {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub max_speed: f32,
    pub max_force: f32,
    pub target: Vec2,
}

impl Wanderer {
    pub fn new(pos: Vec2, vel: Vec2, acc: Vec2, max_speed: f32, max_force: f32, target: Vec2) -> Self {
        Self {
            pos,
            vel,
            acc,
            max_speed,
            max_force,
            target,
        }
    }

    pub fn movement(&mut self) {
        self.vel += self.acc;
        self.vel = self.vel.clamp_length(0.0, self.max_speed);
        self.pos += self.vel;
        self.acc *= 0.0;
    }


    pub fn apply_force(&mut self, force: Vec2) {
        self.acc += force;
    }

    pub fn seek(&mut self, target: Vec2) {
        let desired = target - self.pos;
        let desired = desired.normalize() * self.max_speed;
        let steer = desired - self.vel;
        let steer = steer.clamp_length(0.0, self.max_force);
        self.apply_force(steer);
    }
    
    pub fn set_target(&mut self, target: Vec2) {
        self.target = target;
    }
    
}