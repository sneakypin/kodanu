use crate::Color;

#[derive(Default, Debug, Clone, Copy)]
pub struct Material {
    color: Color,
}

impl Material {
    pub fn color(&self) -> Color {
        self.color
    }
}

impl From<Color> for Material {
    fn from(value: Color) -> Self {
        Self { color: value }
    }
}
