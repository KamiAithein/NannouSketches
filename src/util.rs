use std::cmp::{min, max};
use std::pin::Pin;
use std::thread;

use crate::models::{planet::Planet, planet::PlanetMeta, model::Model, model::State};

use crate::traits::{drawable::Drawable};

extern crate nalgebra as na;
use na::{Unit, Vector2};

use nannou::prelude::*;

use rand::Rng;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};


static dt: f32 = 0.1;
static g: f32 = 1. ;



pub fn dir_vec_from_to(from: &Planet, to: &Planet) -> Unit<Vector2<f32>> {
    Unit::new_normalize(to.pos - from.pos)
}

pub fn area_from_planet(planet: &Planet) -> f32 {
    PI * planet.r.pow(2)
}

// f = ma
pub fn f_from_planets(planet1: &Planet, planet2: &Planet) -> f32 {
    let top = g*(area_from_planet(planet1) * area_from_planet(planet2));
    let bot = planet1.pos.metric_distance(&planet2.pos).pow(2);
    if bot - 0.0_f32 < 0.001_f32 {
        return 0.0; // just in case
    }
    return top/bot; 
}
// planet1: to get accel for
pub fn a_from_planets(planet1: &Planet, planet2: &Planet) -> f32 {
    let a = f_from_planets(planet1, planet2) / planet1.r;
    return a;
}
