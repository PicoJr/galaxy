// ! galaxy controller (handles event)

use graphics::math::{add, dot, mul_scalar, Scalar, square_len, sub, Vec2d};
use piston::input::{Button, Key};
use piston::input::GenericEvent;

use crate::camera::Camera;
use crate::config::Config;
use crate::Galaxy;
use crate::galaxy::Planet;

pub struct GalaxySettings {
    pub gravity: Scalar,
    pub softening_factor: Scalar,
    pub restitution_factor: Scalar,
}

impl GalaxySettings {
    pub fn from_config(config: &Config) -> GalaxySettings {
        GalaxySettings {
            gravity: config.gravity,
            softening_factor: config.softening_factor,
            restitution_factor: config.restitution_factor,
        }
    }
}

/// Handles event for the galaxy simulation
pub struct GalaxyController {
    /// stores the galaxy state
    pub galaxy: Galaxy,
    /// stores the galaxy camera
    pub camera: Camera,
    /// galaxy physic settings
    pub settings: GalaxySettings,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let p0 = Planet::default(0., 0., 2., 0);
        let p1 = Planet::default(1., 0., 2., 0);
        let p2 = Planet::default(3., 0., 2., 0);
        let p3 = Planet::default(5., 0., 1., 0);
        assert!(intersect(&p0, &p1));
        assert!(intersect(&p1, &p2));
        assert!(!intersect(&p0, &p3));
    }
}

/// optimized disc intersection check
/// (optimized) <=> no sqrt computation
fn intersect(planet: &Planet, other_planet: &Planet) -> bool {
    square_len(sub(other_planet.position, planet.position)) <= (other_planet.r + planet.r) * (other_planet.r + planet.r)
}


impl GalaxyController {
    /// Creates a new galaxy controller.
    pub fn new(galaxy: Galaxy, camera: Camera, settings: GalaxySettings) -> GalaxyController {
        GalaxyController {
            galaxy,
            camera,
            settings,
        }
    }

    /// Creates a new galaxy controller from config.
    pub fn from_config(config: &Config) -> GalaxyController {
        GalaxyController::new(Galaxy::from_config(config), Camera::from_config(config), GalaxySettings::from_config(config))
    }

    fn compute_gravitational_force(planet: &Planet, other_planets: &[Planet], gravity: Scalar, softening_factor: Scalar) -> Vec2d<Scalar> {
        other_planets.iter().fold([0.0, 0.0], |acc, p| {
            let acceleration = GalaxyController::compute_single_gravitational_force(planet, p, gravity, softening_factor);
            add(acc, acceleration)
        },
        )
    }

    fn compute_single_gravitational_force(planet: &Planet, other_planet: &Planet, gravity: Scalar, softening_factor: Scalar) -> Vec2d<Scalar> {
        if planet.id == other_planet.id {
            return [0., 0.];
        }
        let sq_distance = square_len(sub(other_planet.position, planet.position));
        let acc = gravity * other_planet.mass / (sq_distance + softening_factor);
        mul_scalar(sub(other_planet.position, planet.position), acc / sq_distance.sqrt())
    }

    fn compute_impulse(planet: &Planet, other_planets: &[Planet], restitution_factor: Scalar, softening_factor: Scalar) -> Vec2d<Scalar> {
        other_planets.iter().fold([0.0, 0.0], |acc, p| {
            let impulse = GalaxyController::compute_single_impulse(planet, p, restitution_factor, softening_factor);
            add(acc, impulse)
        },
        )
    }

    // https://gamedevelopment.tutsplus.com/tutorials/how-to-create-a-custom-2d-physics-engine-the-basics-and-impulse-resolution--gamedev-6331
    fn compute_single_impulse(planet: &Planet, other_planet: &Planet, restitution_factor: Scalar, softening_factor: Scalar) -> Vec2d<Scalar> {
        let dv = sub(other_planet.velocity, planet.velocity);
        let sq_distance = square_len(sub(other_planet.position, planet.position));
        let distance = sq_distance.sqrt();
        let normal = mul_scalar(sub(other_planet.position, planet.position), 1. / (distance + softening_factor));
        let velocity_along_normal = dot(normal, dv);
        let intersect = intersect(planet, other_planet);

        // if velocity along normal > 0 then
        // planets are already separating
        if planet.id == other_planet.id || !intersect || velocity_along_normal > 0. {
            [0., 0.]
        } else {
            let j = -(1. + restitution_factor) * velocity_along_normal;
            let j = j / (planet.imass + other_planet.imass) * planet.imass;
            mul_scalar(normal, -j)
        }
    }

    fn compute_gravitational_interactions(&mut self, dt: f64) {
        for i in 0..self.galaxy.planets.len() {
            let mut acceleration: Vec2d<Scalar> = [0., 0.];
            if let Some(planet) = self.galaxy.planets.get(i) {
                acceleration = GalaxyController::compute_gravitational_force(&planet, &self.galaxy.planets, self.settings.gravity, self.settings.softening_factor);
            }
            if let Some(planet) = self.galaxy.planets.get_mut(i) {
                planet.velocity = add(planet.velocity, mul_scalar(acceleration, dt));
            }
        }
    }

    fn compute_impulses(&mut self) {
        for i in 0..self.galaxy.planets.len() {
            let mut impulse: Vec2d<Scalar> = [0., 0.];
            if let Some(planet) = self.galaxy.planets.get(i) {
                impulse = GalaxyController::compute_impulse(&planet, &self.galaxy.planets, self.settings.restitution_factor, self.settings.softening_factor);
            }
            if let Some(planet) = self.galaxy.planets.get_mut(i) {
                planet.impulse = impulse;
            }
        }
    }

    fn compute_positions(&mut self, dt: f64) {
        for planet in self.galaxy.planets.iter_mut() {
            planet.velocity = add(planet.velocity, planet.impulse);
            planet.position = add(planet.position, mul_scalar(planet.velocity, dt));
        }
    }


    /// Update planet positions according to gravitational interactions.
    pub fn update(&mut self, dt: f64) {
        self.compute_gravitational_interactions(dt);
        self.compute_impulses();
        self.compute_positions(dt);
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        match e.press_args() {
            Some(Button::Keyboard(Key::Left)) => self.camera.position = add(self.camera.position, [-self.camera.camera_speed, 0.]),
            Some(Button::Keyboard(Key::Right)) => self.camera.position = add(self.camera.position, [self.camera.camera_speed, 0.]),
            Some(Button::Keyboard(Key::Up)) => self.camera.position = add(self.camera.position, [0., -self.camera.camera_speed]),
            Some(Button::Keyboard(Key::Down)) => self.camera.position = add(self.camera.position, [0., self.camera.camera_speed]),
            Some(Button::Keyboard(Key::PageDown)) => self.camera.zoom /= self.camera.zoom_factor,
            Some(Button::Keyboard(Key::PageUp)) => self.camera.zoom *= self.camera.zoom_factor,
            _ => ()
        }
    }
}
