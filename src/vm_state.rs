use std::ptr::NonNull;

use crate::{memory_manage::SemiSpaceMemory, object_model::Slot, scope_model::Scope};



#[derive(Debug, Clone)]
pub struct VMState {
    pub heap: SemiSpaceMemory,
    pub current: NonNull<Scope>,
}

impl VMState {
    pub unsafe fn alloc(&mut self, size: usize) -> Slot {
        self.heap.alloc(self.current, size).as_mut()
    }
}