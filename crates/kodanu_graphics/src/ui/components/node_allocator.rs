use crate::NodeId;

#[derive(Debug, Clone, Copy)]
pub struct NodeSlot {
    pub generation: u32,
    pub allocated: bool,
}

impl NodeSlot {
    pub fn new(generation: u32, allocated: bool) -> Self {
        Self {
            generation,
            allocated,
        }
    }
}

pub struct NodeAllocator {
    slots: Vec<NodeSlot>,
    free: Vec<u32>,
}

impl Default for NodeAllocator {
    fn default() -> Self {
        Self {
            slots: vec![NodeSlot::new(0, true)],
            free: Vec::new(),
        }
    }
}

impl NodeAllocator {
    pub fn allocate(&mut self) -> NodeId {
        if let Some(index) = self.free.pop() {
            let slot = &mut self.slots[index as usize];
            slot.allocated = true;

            return NodeId::new(index, slot.generation);
        }

        let index = u32::try_from(self.slots.len()).expect("UI node count exceeded u32::MAX");

        self.slots.push(NodeSlot {
            generation: 0,
            allocated: true,
        });

        NodeId::new(index, 0)
    }

    pub fn deallocate(&mut self, id: NodeId) -> bool {
        if id == NodeId::ROOT {
            return false;
        }

        let Some(slot) = self.slots.get_mut(id.index() as usize) else {
            return false;
        };

        if !slot.allocated || slot.generation != id.generation() {
            return false;
        }

        let Some(generation) = slot.generation.checked_add(1) else {
            panic!("UI node generation exhausted");
        };

        slot.generation = generation;
        slot.allocated = false;
        self.free.push(id.index());

        true
    }

    pub fn contains(&self, id: NodeId) -> bool {
        self.slots
            .get(id.index() as usize)
            .is_some_and(|slot| slot.allocated && slot.generation == id.generation())
    }

    pub fn len(&self) -> usize {
        self.slots.len() - self.free.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
