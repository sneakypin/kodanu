use crate::Handle;

use std::num::NonZeroU32;

pub(crate) struct AssetSlot<A> {
    gens: NonZeroU32,
    asset: Option<A>,
}

impl<A> AssetSlot<A> {
    pub fn new(gens: NonZeroU32, value: Option<A>) -> Self {
        Self { gens, asset: value }
    }
}

pub(crate) struct AssetStorage<A> {
    slots: Vec<AssetSlot<A>>,
    free: Vec<u32>,
}

impl<A> Default for AssetStorage<A> {
    fn default() -> Self {
        Self {
            slots: Vec::default(),
            free: Vec::default(),
        }
    }
}

impl<A> AssetStorage<A> {
    pub fn add(&mut self, asset: A) -> Handle<A> {
        if let Some(id) = self.free.pop() {
            let slot = &mut self.slots[id as usize];

            slot.asset = Some(asset);

            return Handle::new(id, slot.gens);
        }

        let id = self.slots.len() as u32;
        let gens = NonZeroU32::new(1).unwrap();

        self.slots.push(AssetSlot::new(gens, Some(asset)));

        Handle::new(id, gens)
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, id: Handle<A>) -> Option<A> {
        let slot = self.slots.get_mut(id.as_usize())?;

        if slot.gens == id.gens() {
            let asset = slot.asset.take()?;

            slot.gens = NonZeroU32::new(slot.gens.get().wrapping_add(1))
                .unwrap_or_else(|| NonZeroU32::new(1).unwrap());

            self.free.push(id.index());

            return Some(asset);
        }

        None
    }

    pub fn get(&self, id: Handle<A>) -> Option<&A> {
        let slot = self.slots.get(id.as_usize())?;

        if slot.gens == id.gens() {
            return slot.asset.as_ref();
        }

        None
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, id: Handle<A>) -> Option<&mut A> {
        let slot = self.slots.get_mut(id.as_usize())?;

        if slot.gens == id.gens() {
            return slot.asset.as_mut();
        }

        None
    }
}
