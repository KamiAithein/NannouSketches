mod models;
use std::pin::Pin;
use std::thread;

use crate::models::{planet::Planet, planet::PlanetMeta, model::Model, model::State};

mod traits;
use crate::traits::{drawable::Drawable};

extern crate nalgebra as na;
use futures::Future;
use futures::future::{join_all, JoinAll};
use na::{Vector3, Unit, Vector2};

use nannou::prelude::*;

use rand::Rng;

use futures::executor::block_on;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

static dt: f32 = 0.1;
static g: f32 = 0.5 ;

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();

    println!("ended!");
}

fn model(_app: &App) -> Model {
    

    let mut planets: Vec<Planet> = vec![
        // Planet {
        //     pos : Vector3::new(0., 0., 0.),
        //     r: 200.,
        //     v: Vector3::new(0., 0., 0.),
        // },
        // Planet {
        //     pos : Vector3::new(0., 250., 0.0),
        //     r: 10.0,
        //     v: Vector3::new(-1., 0., 0.)
        // },
        // Planet {
        //     pos : Vector3::new(0., -280., 0.0),
        //     r: 25.0,
        //     v: Vector3::new(1., 0., 0.)
        // },
    ];
    let pos_generator = || { 
        let mut rng = rand::thread_rng();
        rng.gen_range(-400.0..400.0)
    };
    let v_generator = || { 
        let mut rng = rand::thread_rng();
        rng.gen_range(-10.0..10.0) 
    };
    let mut rng = rand::thread_rng();

    for _ in 0..300 {
        planets.push(Planet {
            pos: Vector3::new(pos_generator(), pos_generator(), pos_generator()),
            r: rng.gen_range(3.0..8.0),
            v: Vector3::new(v_generator(), v_generator(), v_generator()),
            meta: PlanetMeta {
                is_dead: false
            },
        })
    }

    Model {
        state: State::Start,
        planets,
    }
}


fn area_from_planet(planet: &Planet) -> f32 {
    PI * planet.r.pow(2)
}

// f = ma
fn f_from_planets(planet1: &Planet, planet2: &Planet) -> f32 {
    let top = g*(area_from_planet(planet1) * area_from_planet(planet2));
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

fn handle_planet(mut planet1: Planet, planets_view: Vec<Planet>) -> Planet {
    for planet2_i in 0..planets_view.len() {
        
        let planet2 = planets_view[planet2_i];
        
        if (planet1.pos - planet2.pos).magnitude() < 0.00001 { continue; } //touching
        
        let dist_vec = planet2.pos - planet1.pos;

        if dist_vec.magnitude() <= (planet1.r + planet2.r) { // 0.9 error term
            //on one pass the smaller will be seen
            // the smaller will be set as dead
            //on the second pass the larger will be seen
            // the larger will increase size
            if planet1.r <= planet2.r {
                planet1.meta.is_dead = true;
            } else {
                // conservation of momentum
                planet1.v = (planet1.r * planet1.v + planet2.r * planet2.v) /
                            (planet1.r + planet2.r);
                planet1.r = ((planet1.r.pow(2) + planet2.r.pow(2)) as f32) .sqrt();
            }
        } // touching

        let dir_vec = Unit::new_normalize(dist_vec);

        let a = a_from_planets(&planet1, &planet2);

        let a_vec = dir_vec.scale(a);

        planet1.v = planet1.v + a_vec * dt;
    }

    return planet1;

}

fn handle_planets(planets: Vec<Planet>) -> Vec<Planet> {
    
    let planets_view = planets.clone();
    let mut planets_new: Vec<Planet> = planets
        .into_iter()
        .map(|planet| (planet, planets_view.clone()))
        .collect::<Vec<(Planet, Vec<Planet>)>>()
        .into_par_iter()
        .map(|(planet, planet_view)| handle_planet(planet, planet_view))
        .collect();
    
    let mut i = 0;
    while i < planets_new.len() {
        if planets_new[i].meta.is_dead {
            planets_new.remove(i);
            
        } else {
            planets_new[i].pos = planets_new[i].pos + planets_new[i].v * dt;
            i += 1;
        }
    }

    return planets_new;


}

// fn handle_planets(model: &mut Model) {
//     let planets = &mut model.planets;
//     let planets_view = planets.clone();

//     for planet1_i in 0..planets.len() {
        
//         for planet2_i in 0..planets_view.len() {
//             if planet1_i == planet2_i { continue; }
            
//             let planet1: &mut Planet = &mut planets[planet1_i];
//             let planet2 = planets_view[planet2_i];

            
//             let dist_vec = planet2.pos - planet1.pos;

//             if dist_vec.magnitude() <= (planet1.r + planet2.r) { // 0.9 error term
//                 //on one pass the smaller will be seen
//                 // the smaller will be set as dead
//                 //on the second pass the larger will be seen
//                 // the larger will increase size
//                 if planet1.r <= planet2.r {
//                     planet1.meta.is_dead = true;
//                 } else {
//                     // conservation of momentum
//                     planet1.v = (planet1.r * planet1.v + planet2.r * planet2.v) /
//                                 (planet1.r + planet2.r);
//                     planet1.r += planet2.r.sqrt().sqrt();
//                 }
//             } // touching

//             let dir_vec = Unit::new_normalize(dist_vec);

//             let a = a_from_planets(&planet1, &planet2);

//             let a_vec = dir_vec.scale(a);

//             planet1.v = planet1.v + a_vec * dt;
//         }
//     }


//     // could be 1 loop

//     let mut i = 0;
//     while (i < planets.len()) {
//         if planets[i].meta.is_dead {
//             planets.remove(i);
            
//         } else {
//             planets[i].pos = planets[i].pos + planets[i].v * dt;
//             i += 1;
//         }
//     }

// }

fn update(app: &App, model: &mut Model, _update: Update) {
    model.planets = handle_planets(model.planets.clone());

    
    // model.orbit = Vector3::new(app.mouse.x, app.mouse.y, 0.);
}

fn handle_window_event(wevent: WindowEvent, app: &App, model: &mut Model) {
    match wevent {
        MousePressed(MouseButton::Left) => {
            match model.state {
                State::Start => {
                    model.state = State::CreateStart(app.mouse.x, app.mouse.y);
                },
                State::VelStart(x, y, r) => {
                    let edge = Vector2::new(app.mouse.x, app.mouse.y);
                    let origin = Vector2::new(x, y);
                    let v_rad = origin.metric_distance(&edge);

                    let dir_vec = Unit::new_normalize(edge - origin);
                    let v_pre = dir_vec.scale(v_rad);
                    let v = Vector3::new (v_pre.x, v_pre.y, 0.);
                    
                    model.planets.push(Planet {
                        pos: Vector3::new(x, y, 0.),
                        r,
                        v: v.scale(0.01),
                        meta: PlanetMeta {
                            is_dead: false
                        },
                    });

                    model.state = State::Start;
                },
                _ => {}
            }
        },
        MouseReleased(MouseButton::Left) => {
            match(model.state) {
                State::CreateStart(x, y) => {
                    model.state = State::SizeEnd(x, y, app.mouse.x, app.mouse.y);

                    let origin = Vector2::new(x, y);
                    let edge_pt = Vector2::new(app.mouse.x, app.mouse.y);
                    let dist = origin.metric_distance(&edge_pt);
        
                    model.state = State::VelStart(x, y, dist);

                }
                _ => {}
            };

        }
        _ => {}
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id,
            simple: Some(wevent),
            // TODO: Re-add this when winit#1387 is resolved.
            // raw: winit::event::WindowEvent,
        } => handle_window_event(wevent, app, model),
        _ => {} 
    }

}


fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(PURPLE);

    match model.state {
        State::CreateStart(x, y) => {
            let origin = Vector2::new(x, y);
            let cursor = Vector2::new(app.mouse.x, app.mouse.y);

            let r = cursor.metric_distance(&origin);
            draw.ellipse().xy(Vec2::new(x, y)).radius(r);
        },
        State::SizeEnd(x0, y0, x1, y1) => {
            let origin = Vector2::new(x0, y0);
            let edge = Vector2::new(x1, y1);

            let r = origin.metric_distance(&edge);
            draw.ellipse().xy(Vec2::new(x0, y0)).radius(r);
        }
        State::VelStart(x, y, r) => {
            draw.ellipse().xy(Vec2::new(x, y)).radius(r);
            draw.arrow()
                .start(Vec2::new(x, y))
                .end(Vec2::new(app.mouse.x, app.mouse.y))
                .color(BLUE)
                .stroke_weight(1.)
            ;
        }
        _ => {}
    }

    model.planets.iter().for_each(|drawable|{drawable.draw(&draw)});
    
    draw.to_frame(app, &frame).unwrap();
}