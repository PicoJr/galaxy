//! Galaxy view (does all the drawing)

use graphics::{Context, Image};
use graphics::Ellipse;
use graphics::ellipse::circle;
use graphics::types::Color;
use opengl_graphics::{GlGraphics, Texture};
use piston_window::TextureSettings;

use crate::config::Config;
pub use crate::galaxy_controller::GalaxyController;

/// Galaxy view settings
pub struct GalaxyViewSettings {
    /// planet color
    planet_color: Color,
    /// planet texture
    planet_texture: Option<Texture>,
}

impl GalaxyViewSettings {
    /// Creates a new galaxy view settings
    pub fn default() -> GalaxyViewSettings {
        GalaxyViewSettings::from_config(&Config::default())
    }

    /// Creates a new galaxy view settings from config
    pub fn from_config(config: &Config) -> GalaxyViewSettings {
        let texture = match &config.planet_texture_path {
            None => None,
            Some(path_string) =>
                match Texture::from_path(path_string, &TextureSettings::new()) {
                    Ok(t) => Some(t),
                    Err(_) => None,
                }
        };
        GalaxyViewSettings {
            planet_color: config.planet_color,
            planet_texture: texture,
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
    pub fn draw(&self, controller: &GalaxyController, c: &Context, g: &mut GlGraphics) {
        let settings = &self.settings;

        // Nb: IDE borrow checker complains about 'c' but code compiles just fine.
        let planets = &controller.galaxy.planets;
        let disc = Ellipse::new(settings.planet_color);

        let image = Image::new();

        let transform = controller.camera.world_to_view_transform(c.transform);

        for planet in planets.iter() {
            if let Some(texture) = &settings.planet_texture {
                image.rect(circle(planet.position[0], planet.position[1], planet.r)).draw(texture, &c.draw_state, transform, g);
            } else {
                disc.draw(circle(planet.position[0], planet.position[1], planet.r), &c.draw_state, transform, g);
            }
        }
    }
}