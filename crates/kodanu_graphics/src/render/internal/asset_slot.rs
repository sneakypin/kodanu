#![allow(dead_code)]

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

impl<A> AssetSlot<A> {
    pub fn get(&self) -> Option<&A> {
        self.asset.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut A> {
        self.asset.as_mut()
    }

    pub fn take(&mut self) -> Option<A> {
        self.asset.take()
    }

    pub fn gens(&self) -> NonZeroU32 {
        self.gens
    }

    pub fn set(&mut self, value: Option<A>) {
        self.asset = value
    }

    pub fn set_gens(&mut self, gens: NonZeroU32) {
        self.gens = gens
    }
}
