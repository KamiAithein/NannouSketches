mod models;
use crate::models::{planet::Planet, model::Model};

mod traits;
use crate::traits::{drawable::Drawable};

extern crate nalgebra as na;
use na::{Vector3, Unit};

use nannou::prelude::*;

use rand::Rng;

static dt: f32 = 1.0;
static g: f32 = 1.5;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();

    println!("ended!");
}

fn model(_app: &App) -> Model {
    

    let mut planets: Vec<Planet> = vec![
        Planet {
            pos : Vector3::new(0., 0., 0.),
            r: 200.,
            v: Vector3::new(0., 0., 0.),
        },
        Planet {
            pos : Vector3::new(0., 300., 0.0),
            r: 10.0,
            v: Vector3::new(-1., 0., 0.)
        }
    ];
    // let pos_generator = || { 
    //     let mut rng = rand::thread_rng();
    //     rng.gen_range(-200.0..200.0)
    // };
    // let v_generator = || { 
    //     let mut rng = rand::thread_rng();
    //     rng.gen_range(-0.2..0.2) 
    // };
    // let mut rng = rand::thread_rng();

    // for i in (0..50) {
    //     planets.push(Planet {
    //         pos: Vector3::new(pos_generator(), pos_generator(), pos_generator()),
    //         r: rng.gen_range(5.0..30.0),
    //         v: Vector3::new(v_generator(), v_generator(), v_generator())
    //     })
    // }

    Model {
        planets
    }
}



// f = ma
fn f_from_planets(planet1: &Planet, planet2: &Planet) -> f32 {
    let top = g*(planet2.r * planet1.r);
    let bot = planet1.pos.metric_distance(&planet2.pos).pow(2);
    if bot - 0.0_f32 < 0.001_f32 {
        return 0.0; // just in case
    }
    return top/bot; 
}
// planet1: to get accel for
fn a_from_planets(planet1: &Planet, planet2: &Planet) -> f32 {
    let a = f_from_planets(planet1, planet2) / planet1.r;
    return a;
}



fn handle_planets(model: &mut Model) {
    
    for planet1_i in 0..model.planets.len() {
        for planet2_i in 0..model.planets.len() {
            if planet1_i == planet2_i { continue; }
            
            let planet2 = model.planets[planet2_i].clone();

            let planet1: &mut Planet = &mut model.planets[planet1_i];
            
            let dist_vec = planet2.pos - planet1.pos;

            if dist_vec.magnitude() <= planet1.r + planet2.r {continue} // touching

            let dir_vec = Unit::new_normalize(dist_vec);

            let a = a_from_planets(&planet1, &planet2);

            let a_vec = dir_vec.scale(a);

            planet1.v = planet1.v + a_vec * dt;
            
            planet1.pos = planet1.pos + planet1.v * dt;
        }
    }

}

fn update(app: &App, model: &mut Model, _update: Update) {
    handle_planets(model);
    
    // model.orbit = Vector3::new(app.mouse.x, app.mouse.y, 0.);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(PURPLE);

    model.planets.iter().for_each(|drawable|{drawable.draw(&draw)});
    
    draw.to_frame(app, &frame).unwrap();
}