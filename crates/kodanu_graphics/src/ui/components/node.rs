use crate::{NodeId, Rect, Style};

pub struct Node {
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    style: Style,
    rect: Rect,
}

impl Node {
    pub fn new(style: Style) -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            style,
            rect: Rect::ZERO,
        }
    }
}

impl Node {
    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }

    pub fn children(&self) -> &[NodeId] {
        &self.children
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn set_parent(&mut self, id: Option<NodeId>) {
        self.parent = id
    }

    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: NodeId) {
        self.children.retain(|&id| id != child);
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect
    }
}
