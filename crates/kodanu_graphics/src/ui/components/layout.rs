use crate::{Direction, NodeId, Rect, Spacing, UiContext, UiSize};

use kodanu_math::Vec2;

#[derive(Debug, Default)]
pub struct Layout;

impl Layout {
    pub fn compute(context: &mut UiContext, size: Vec2) {
        let root = context.root();
        let rect = Rect::new(0.0, 0.0, size.x.max(0.0), size.y.max(0.0));

        Self::layout_node(context, root, rect);
    }

    fn layout_node(context: &mut UiContext, id: NodeId, rect: Rect) {
        context.set_rect(id, rect);

        let style = *context.get(id).expect("layout node must exist").style();

        let content_rect = Self::content_rect(rect, style.padding);

        let children = context
            .get(id)
            .expect("layout node must exist")
            .children()
            .to_vec();

        if children.is_empty() {
            return;
        }

        Self::layout_children(context, &children, content_rect, style.direction);
    }

    fn layout_children(
        context: &mut UiContext,
        children: &[NodeId],
        parent: Rect,
        direction: Direction,
    ) {
        let mut fixed_main = 0.0;
        let mut fill_count = 0usize;

        for &child in children {
            let style = *context.get(child).expect("layout child must exist").style();

            let (main_size, margin_before, margin_after, _is_fill) = match direction {
                Direction::Vertical => (
                    style.height,
                    style.margin.top,
                    style.margin.bottom,
                    matches!(style.height, UiSize::Fill),
                ),
                Direction::Horizontal => (
                    style.width,
                    style.margin.left,
                    style.margin.right,
                    matches!(style.width, UiSize::Fill),
                ),
            };

            fixed_main += margin_before + margin_after;

            match main_size {
                UiSize::Fixed(value) => {
                    fixed_main += value.max(0.0);
                }
                UiSize::Fill => {
                    fill_count += 1;
                }
            }
        }

        let parent_main = match direction {
            Direction::Vertical => parent.height,
            Direction::Horizontal => parent.width,
        };

        let remaining = (parent_main - fixed_main).max(0.0);

        let fill_size = if fill_count > 0 {
            remaining / fill_count as f32
        } else {
            0.0
        };

        let mut cursor = match direction {
            Direction::Vertical => parent.y,
            Direction::Horizontal => parent.x,
        };

        for &child in children {
            let style = *context.get(child).expect("layout child must exist").style();

            let rect = Self::child_rect(style, parent, direction, fill_size, &mut cursor);

            Self::layout_node(context, child, rect);
        }
    }

    fn child_rect(
        style: crate::Style,
        parent: Rect,
        direction: Direction,
        fill_size: f32,
        cursor: &mut f32,
    ) -> Rect {
        let width = match style.width {
            UiSize::Fixed(value) => value.max(0.0),
            UiSize::Fill => (parent.width - style.margin.horizontal()).max(0.0),
        };

        let height = match style.height {
            UiSize::Fixed(value) => value.max(0.0),
            UiSize::Fill => (parent.height - style.margin.vertical()).max(0.0),
        };

        match direction {
            Direction::Vertical => {
                let main_size = match style.height {
                    UiSize::Fixed(_) => height,
                    UiSize::Fill => fill_size,
                };

                let x = parent.x + style.margin.left + style.position.x;
                let y = *cursor + style.margin.top + style.position.y;

                *cursor += style.margin.top + main_size + style.margin.bottom;

                Rect::new(x, y, width, main_size)
            }

            Direction::Horizontal => {
                let main_size = match style.width {
                    UiSize::Fixed(_) => width,
                    UiSize::Fill => fill_size,
                };

                let x = *cursor + style.margin.left + style.position.x;
                let y = parent.y + style.margin.top + style.position.y;

                *cursor += style.margin.left + main_size + style.margin.right;

                Rect::new(x, y, main_size, height)
            }
        }
    }

    fn content_rect(rect: Rect, padding: Spacing) -> Rect {
        Rect::new(
            rect.x + padding.left,
            rect.y + padding.top,
            (rect.width - padding.horizontal()).max(0.0),
            (rect.height - padding.vertical()).max(0.0),
        )
    }
}
