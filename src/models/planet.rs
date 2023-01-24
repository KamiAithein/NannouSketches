extern crate nalgebra as na;
use na::{Vector3, Unit};

use crate::traits::{drawable::Drawable};
use crate::models::model::Model;

use nannou::prelude::*;

#[derive (Clone, Copy, Debug)]
pub struct Planet {
    pub pos: Vector3<f32>,
    pub r: f32,
    pub v: Vector3<f32>,
    pub meta: PlanetMeta,
}

#[derive (Clone, Copy, Debug)]
pub struct PlanetMeta {
    pub is_dead: bool
}

impl Drawable for Planet {
    fn draw(&self, draw: &Draw) {
        draw
            .ellipse()
            .xyz(Vec3::new(self.pos.x, self.pos.y, self.pos.z))
            .radius(self.r)
        ;
        draw
            .arrow()
            .start(Vec2::new(self.pos.x, self.pos.y))
            .end(Vec2::new(self.pos.x, self.pos.y) + Vec2::new(self.v.x, self.v.y) * 100.)
            .color(BLUE)
            .stroke_weight(10.)
        ;
    }
}