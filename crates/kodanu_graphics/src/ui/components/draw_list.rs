use crate::Quad;

#[derive(Default)]
pub struct DrawList {
    quads: Vec<Quad>,
}

impl DrawList {
    pub fn clear(&mut self) {
        self.quads.clear();
    }

    pub fn push(&mut self, quad: Quad) {
        self.quads.push(quad);
    }

    pub fn quads(&self) -> &[Quad] {
        &self.quads
    }

    pub fn len(&self) -> usize {
        self.quads.len()
    }

    pub fn is_empty(&self) -> bool {
        self.quads.is_empty()
    }
}
