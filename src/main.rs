#![warn(missing_docs)]

//! Galaxy simulation

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::fs::File;

use glutin_window::GlutinWindow;
use graphics::clear;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{Events, EventSettings};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

pub use crate::camera::Camera;
use crate::config::Config;
pub use crate::galaxy::Galaxy;
pub use crate::galaxy_controller::GalaxyController;
pub use crate::galaxy_view::{GalaxyView, GalaxyViewSettings};

mod galaxy;
mod galaxy_controller;
mod galaxy_view;
mod camera;
mod config;

fn main() {
    let opengl = OpenGL::V3_2;
    let config = match File::open("res/config.json") {
        Ok(json_file) => Config::from_json(json_file),
        Err(_) => {
            println!("could not open res/config.json, using default config instead");
            Config::default()
        },
    };
    let settings = WindowSettings::new("Galaxy", config.window_size)
        .opengl(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build()
      .expect("Could not create window");

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);


    let mut galaxy_controller = GalaxyController::from_config(&config);
    let galaxy_view = GalaxyView::from_config(&config);

    while let Some(e) = events.next(&mut window) {
        galaxy_controller.event(&e);
        galaxy_controller.update(config.frame_time_step);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                clear(config.background_color, g);
                galaxy_view.draw(&galaxy_controller, &c, g);
            });
        }
    }

    println!("{}", settings.get_exit_on_esc());
}
