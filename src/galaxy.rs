use graphics::math::{Scalar, Vec2d};

use crate::config::Config;

#[derive(Clone, Copy)]
pub struct Planet {
    /// x, y position
    pub position: Vec2d<Scalar>,
    /// x, y velocity
    pub velocity: Vec2d<Scalar>,
    /// x, y impulse (for collisions)
    pub impulse: Vec2d<Scalar>,
    /// radius
    pub r: Scalar,
    /// mass > 0
    pub mass: Scalar,
    /// 1 / mass
    pub imass: Scalar,
    /// unique id
    pub id: usize,

}

impl Planet {
    /// create a new planet
    /// radius must be > 0.0
    pub fn default(x: Scalar, y: Scalar, r: Scalar, id: usize) -> Planet {
        assert!(r > 0.);
        let mass = r * r;
        Planet {
            position: [x, y],
            velocity: [0., 0.],
            impulse: [0., 0.],
            r,
            mass,
            imass: 1. / mass,
            id,
        }
    }
}

/// A galaxy with planets
pub struct Galaxy {
    /// planets
    pub planets: Vec<Planet>,
}

impl Galaxy {
    /// Creates a new galaxy.
    pub fn default() -> Galaxy {
        Galaxy::from_config(&Config::default())
    }

    /// Creates a new galaxy from config.
    pub fn from_config(config: &Config) -> Galaxy {
        Galaxy {
            planets: config.planets.clone()
        }
    }
}