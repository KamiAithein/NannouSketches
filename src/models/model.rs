use crate::models::{planet::Planet};
use crate::traits::{drawable::Drawable};

extern crate nalgebra as na;
use na::{Vector3, Unit};


pub struct Model {
    pub planets: Vec<Planet>,
    pub state: State,
}

#[derive (Debug)]
pub enum State {
    Start,
    CreateStart(f32, f32),
    SizeEnd(f32, f32, f32, f32),
    VelStart(f32, f32, f32),
    VelEnd(f32, f32, f32, Vector3<f32>)
}