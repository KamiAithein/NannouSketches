extern crate nalgebra as na;
use na::{Vector2, Unit};

use crate::traits::{drawable::Drawable};
use crate::models::model::Model;

use nannou::prelude::*;

#[derive (Clone, Copy, Debug)]
pub struct Planet {
    pub pos: Vector2<f32>,
    pub r: f32,
    pub v: Vector2<f32>,
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
            .xy(Vec2::new(self.pos.x, self.pos.y))
            .radius(self.r)
            .color(Rgb::new((self.v.magnitude() * 10.) as u8 % 255,128_u8,0_u8))
        ;
        draw
            .arrow()
            .start(Vec2::new(self.pos.x, self.pos.y))
            .end(Vec2::new(self.pos.x, self.pos.y) + Vec2::new(self.v.x, self.v.y) * 10.)
            .color(BLUE)
            .stroke_weight(1.)
        ;
    }
}