//! Galaxy view (does all the drawing)

use graphics::{Context, Graphics};
use graphics::Ellipse;
use graphics::ellipse::circle;
use graphics::Transformed;
use graphics::types::Color;

use crate::config::Config;
pub use crate::galaxy_controller::GalaxyController;

/// Galaxy view settings
pub struct GalaxyViewSettings {
    /// planet color
    planet_color: Color,
}

impl GalaxyViewSettings {
    /// Creates a new galaxy view settings
    pub fn default() -> GalaxyViewSettings {
        GalaxyViewSettings::from_config(&Config::default())
    }

    /// Creates a new galaxy view settings from config
    pub fn from_config(config: &Config) -> GalaxyViewSettings {
        GalaxyViewSettings {
            planet_color: config.planet_color
        }
    }
}

/// Stores visual information about a galaxy.
pub struct GalaxyView {
    /// stores galaxy view settings
    pub settings: GalaxyViewSettings
}

impl GalaxyView {
    /// creates a new Galaxy view.
    pub fn new(settings: GalaxyViewSettings) -> GalaxyView {
        GalaxyView {
            settings,
        }
    }

    /// Creates a default galaxy view
    pub fn default() -> GalaxyView {
        GalaxyView::new(GalaxyViewSettings::default())
    }

    /// Creates a galaxy view from config
    pub fn from_config(config: &Config) -> GalaxyView {
        GalaxyView::new(GalaxyViewSettings::from_config(config))
    }

    /// Draw galaxy
    pub fn draw<G: Graphics>(&self, controller: &GalaxyController, c: &Context, g: &mut G) {
        let settings = &self.settings;

        // Nb: IDE borrow checker complains about 'c' but code compiles just fine.
        let planets = &controller.galaxy.planets;
        let disc = Ellipse::new(settings.planet_color);

        let transform = c.trans(-controller.camera.position[0], -controller.camera.position[1]).zoom(controller.camera.zoom).transform;

        for planet in planets.iter() {
            disc.draw(circle(planet.position[0], planet.position[1], planet.r), &c.draw_state, transform, g);
        }
    }
}