use crate::models::{planet::Planet};
use crate::traits::{drawable::Drawable};

extern crate nalgebra as na;
use na::{Vector3, Unit};


pub struct Model {
    pub planets: Vec<Planet>,
}