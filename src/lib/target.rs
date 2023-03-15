use bevy::prelude::*;
use rand::Rng;

use crate::SCREEN_WIDTH;

use super::Wanderer;

#[derive(Component, Resource)]
pub struct Target {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub max_speed: f32,
    pub max_force: f32,
    pub radius: f32,
    pub theta: f32,
}

impl Target {
    pub fn new(pos: Vec2, vel: Vec2, acc: Vec2, max_speed: f32, max_force: f32, radius: f32) -> Self {
        Self {
            pos,
            vel,
            acc,
            max_speed,
            max_force,
            radius,
            theta: std::f32::consts::PI / -2.0,
        }
    }

    pub fn movement(&mut self) {
        self.vel += self.acc;
        self.vel = self.vel.clamp_length(0.0, self.max_speed);
        self.pos += self.vel;
        self.acc *= 0.0;

        self.avoid_wall();
    }

    fn avoid_wall(&mut self) {
        if self.pos.x > SCREEN_WIDTH / 2.0 {
            self.theta = std::f32::consts::PI / 1.0;
        } else if self.pos.x < -SCREEN_WIDTH / 2.0 {
            self.theta = 0.0;
        } else if self.pos.y > SCREEN_WIDTH / 2.0 {
            self.theta = std::f32::consts::PI / -2.0;
        } else if self.pos.y < -SCREEN_WIDTH / 2.0 {
            self.theta = std::f32::consts::PI / 2.0;
        }
    }

    pub fn shift_theta(&mut self, wanderer: &mut Wanderer) {
        let mut rng = rand::thread_rng();
        self.theta = self.theta + rng.gen_range(-0.3..0.3);
    
        let target_x = self.radius * self.theta.cos();
        let target_y = self.radius * self.theta.sin();

        let new_pos = Vec2::new(target_x + wanderer.pos.x, target_y + wanderer.pos.y);

        self.pos = new_pos;
    }


}