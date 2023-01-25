use nannou::prelude::*;

use crate::models::model::Model;

pub trait Drawable {
    fn draw(&self, draw: &Draw, model: &Model);
}