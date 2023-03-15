use bevy::prelude::*;

#[derive(Component)]
pub struct Circle;

#[derive(Resource)]
pub struct Path {
    pub points: Vec<Vec2>,
}

impl Path {
    pub fn new() -> Self {
        Self { points: vec![
            Vec2::new(0.0, 0.0),
        ] }
    }

    pub fn add_point(&mut self, point: Vec2) {
        self.points.push(point);
    }
}