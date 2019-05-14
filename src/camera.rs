//! Camera: position and zoom

use graphics::math::{identity, Matrix2d, Scalar, transform_pos, Vec2d};
use graphics::Transformed;

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
    /// Creates default camera.
    pub fn default() -> Camera {
        Camera::from_config(&Config::default())
    }

    /// Creates a new Camera from config.
    pub fn from_config(config: &Config) -> Camera {
        Camera {
            zoom: config.default_zoom,
            position: config.camera_position,
            zoom_factor: config.zoom_factor,
            camera_speed: config.camera_speed,
        }
    }

    /// convert world to view
    pub fn world_to_view_transform(&self, transform: Matrix2d) -> Matrix2d {
        transform.trans(-self.position[0], -self.position[1]).zoom(self.zoom)
    }

    /// convert view (screen) position to world position
    pub fn view_to_world_position(&self, position: Vec2d<Scalar>) -> Vec2d<Scalar> {
        let transform = identity();
        let transform = transform.zoom(1. / self.zoom).trans(self.position[0], self.position[1]);
        transform_pos(transform, position)
    }

}