use crate::{Color, NodeId, Rect};

pub struct Quad {
    pub rect: Rect,
    pub color: Color,
    pub node: NodeId,
}

impl Quad {
    pub fn new(rect: Rect, color: Color, id: NodeId) -> Self {
        Self {
            rect,
            color,
            node: id,
        }
    }
}
