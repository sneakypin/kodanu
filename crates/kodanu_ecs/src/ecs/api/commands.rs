use crate::{Bundle, Entity, Resource, SystemParam, WorldCell};

#[derive(Clone, Copy)]
pub struct Commands<'w> {
    world: WorldCell<'w>,
}

impl<'w> Commands<'w> {
    pub fn new(world: WorldCell<'w>) -> Commands<'w> {
        Self { world }
    }
}

impl<'w> Commands<'w> {
    pub fn spawn_bundle<B: Bundle>(&self, bundle: B) -> Entity {
        self.world.spawn_bundle(bundle)
    }

    pub fn with_res<R: Resource>(&self, resource: R) {
        self.world.with_res(resource);
    }
}

impl SystemParam for Commands<'_> {
    type Item<'w> = Commands<'w>;

    fn fetch<'w>(world: WorldCell<'w>) -> Self::Item<'w> {
        Commands::new(world)
    }
}
