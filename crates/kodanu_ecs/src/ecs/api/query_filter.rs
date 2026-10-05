use crate::WorldCell;

pub trait QueryFilter {
    type Storage<'w>;

    fn fetch<'w>(cell: WorldCell<'w>) -> Self::Storage<'w>;

    fn matches(storage: &Self::Storage<'_>, entity: u32) -> bool;
}

impl QueryFilter for () {
    type Storage<'w> = ();

    fn fetch<'w>(_cell: WorldCell<'w>) -> Self::Storage<'w> {}

    fn matches(_: &Self::Storage<'_>, _: u32) -> bool {
        true
    }
}
