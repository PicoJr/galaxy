//! Camera: position and zoom

use graphics::math::{Scalar, Vec2d};

use crate::config::Config;

/// Galaxy Camera
pub struct Camera {
    /// zoom: scalar, > 0
    /// 0.5: objects appear twice smaller
    /// 1.0: normal zoom
    /// 2.0: objects appear twice bigger
    pub zoom: Scalar,
    /// screen center == camera position
    pub position: Vec2d<Scalar>,
    /// zoom/de-zoom factor > 1.0
    /// 2.0: zoom => 2 times bigger, de-zoom => 2 times smaller
    pub zoom_factor: Scalar,
    /// distance/frame
    pub camera_speed: Scalar,
}

impl Camera {
    /// returns default camera
    pub fn default() -> Camera {
        Camera::from_config(&Config::default())
    }

    pub fn from_config(config: &Config) -> Camera {
        Camera {
            zoom: config.default_zoom,
            position: config.camera_position,
            zoom_factor: config.zoom_factor,
            camera_speed: config.camera_speed,
        }
    }
}