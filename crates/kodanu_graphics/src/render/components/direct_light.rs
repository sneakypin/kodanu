use crate::Color;

use {kodanu_ecs::Component, kodanu_math::Vec3};

#[derive(Component, Debug)]
pub struct DirectLight {
    direction: Vec3,
    color: Color,
    intensity: f32,
}

impl Default for DirectLight {
    fn default() -> Self {
        Self {
            direction: Vec3::ZERO,
            color: Color::WHITE,
            intensity: 1.0,
        }
    }
}

impl DirectLight {
    pub fn new(direction: Vec3, color: Color, intensity: f32) -> Self {
        Self {
            direction,
            color,
            intensity,
        }
    }
}

impl DirectLight {
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    pub fn set_direction(&mut self, direction: Vec3) {
        self.direction = direction
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity
    }
}
