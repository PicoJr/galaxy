//! Global Configuration

extern crate serde_json;

use std::fs::File;

use graphics::math::{Scalar, Vec2d};
use graphics::types::Color;

use crate::galaxy::Planet;

pub struct Config {
    /// G, the gravitational constant: the higher the stronger gravity will be
    pub gravity: Scalar,
    /// a small positive value to avoid dividing by 0 e.g. 0.01
    pub softening_factor: Scalar,
    /// restitution factor >= 0.0: "bounciness" of objects
    /// 0.0: no bounce
    pub restitution_factor: Scalar,
    /// zoom/de-zoom factor > 1.0
    /// 2.0: zoom => 2 times bigger, de-zoom => 2 times smaller
    pub zoom_factor: Scalar,
    /// default zoom
    pub default_zoom: Scalar,
    /// camera_speed > 0.0
    /// distance/frame
    pub camera_speed: Scalar,
    /// camera starting position
    pub camera_position: Vec2d<Scalar>,
    /// window size (pixels)
    pub window_size: [u32; 2],
    /// frame time step > 0.0
    /// the higher the faster the simulation will be.
    /// NB: collisions may fail if value is too high relatively to objects velocity
    pub frame_time_step: Scalar,
    /// background color (float RGBA)
    /// e.g. [1., 0., 0., 0.8] => RED with 0.8 opacity
    pub background_color: Color,
    /// planet color
    /// e.g. [1., 0., 0., 0.8] => RED with 0.8 opacity
    pub planet_color: Color,
    /// planets
    pub planets: Vec<Planet>,
}

impl Config {
    pub fn default() -> Config {
        let mut planets: Vec<Planet> = vec![];
        planets.push(Planet::default(0., 20., 5., 0));
        planets.push(Planet::default(0., 40., 10., 1));
        planets.push(Planet::default(0., 80., 20., 2));
        Config {
            gravity: 0.05,
            softening_factor: 0.01,
            restitution_factor: 0.2,
            zoom_factor: 2.0,
            default_zoom: 1.0,
            camera_speed: 4.0,
            camera_position: [0., 0.],
            window_size: [512; 2],
            frame_time_step: 0.1,
            background_color: [0.2, 0.2, 0.2, 1.0],
            planet_color: [1.0, 0.6, 0.0, 1.0],
            planets
        }
    }

    fn get_scalar_or(json: &serde_json::Value, default_value: Scalar) -> Scalar {
        match json {
            serde_json::Value::Number(x) => x.as_f64().unwrap(),
            _ => default_value
        }
    }

    fn get_vector2d_or(json: &serde_json::Value, default_value: Vec2d<Scalar>) -> Vec2d<Scalar> {
        let x_value = &json["x"];
        let y_value = &json["y"];
        match (x_value, y_value) {
            (serde_json::Value::Number(x), serde_json::Value::Number(y)) => [x.as_f64().unwrap(), y.as_f64().unwrap()],
            _ => default_value
        }
    }

    fn get_rgba_or(json: &serde_json::Value, default_value: Color) -> Color {
        let r_value = &json["r"];
        let g_value = &json["g"];
        let b_value = &json["b"];
        let a_value = &json["a"];
        match (r_value, g_value, b_value, a_value) {
            (serde_json::Value::Number(r), serde_json::Value::Number(g), serde_json::Value::Number(b), serde_json::Value::Number(a)) =>
                [r.as_f64().unwrap() as f32, g.as_f64().unwrap() as f32, b.as_f64().unwrap() as f32, a.as_f64().unwrap() as f32],
            _ => default_value
        }
    }

    fn get_planets_or(json: &serde_json::Value, default_value: Vec<Planet>) -> Vec<Planet> {
        match json {
            serde_json::Value::Array(planets_value_vec) => {
                let mut i: usize = 0;
                let mut planets: Vec<Planet> = vec![];
                for planet_value in planets_value_vec {
                    let x_value = &planet_value["x"];
                    let y_value = &planet_value["y"];
                    let r_value = &planet_value["r"];
                    match (x_value, y_value, r_value) {
                        (serde_json::Value::Number(x), serde_json::Value::Number(y), serde_json::Value::Number(r)) => {
                            planets.push(Planet::default(x.as_f64().unwrap(), y.as_f64().unwrap(), r.as_f64().unwrap(), i));
                            i += 1;
                        }
                        _ => { println!("error while parsing planets") }
                    }
                }
                planets
            }
            _ => default_value
        }
    }

    pub fn from_json(json_file: File) -> Config {
        let default_config = Config::default();
        let json: serde_json::Value = serde_json::from_reader(json_file).expect("file should have proper JSON");
        Config {
            gravity: Config::get_scalar_or(&json["gravity"], default_config.gravity),
            softening_factor: Config::get_scalar_or(&json["softening_factor"], default_config.softening_factor),
            restitution_factor: Config::get_scalar_or(&json["restitution_factor"], default_config.restitution_factor),
            zoom_factor: Config::get_scalar_or(&json["zoom_factor"], default_config.zoom_factor),
            default_zoom: Config::get_scalar_or(&json["default_zom"], default_config.default_zoom),
            camera_speed: Config::get_scalar_or(&json["camera_speed"], default_config.camera_speed),
            camera_position: Config::get_vector2d_or(&json["camera_position"], default_config.camera_position),
            window_size: default_config.window_size,
            frame_time_step: Config::get_scalar_or(&json["frame_time_step"], default_config.frame_time_step),
            background_color: Config::get_rgba_or(&json["background_color"], default_config.background_color),
            planet_color: Config::get_rgba_or(&json["planet_color"], default_config.planet_color),
            planets: Config::get_planets_or(&json["planets"], default_config.planets),
        }
    }
}