use crate::WorldCell;

pub trait SystemParam {
    type Item<'w>;

    fn fetch<'w>(world: WorldCell<'w>) -> Self::Item<'w>;
}

impl SystemParam for () {
    type Item<'w> = ();

    fn fetch<'w>(_: WorldCell<'w>) -> Self::Item<'w> {}
}
