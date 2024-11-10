use std::ptr::NonNull;

use crate::{
    memory_manage::SemiSpaceMemory,
    object_model::{ObjectHead, ObjectTag, Slot, Symbol},
    scope_model::Scope,
};

#[derive(Debug, Clone)]
pub struct VMState {
    pub heap: SemiSpaceMemory,
    pub current: NonNull<Scope>,
}

impl VMState {
    pub unsafe fn alloc(&mut self, size: usize) -> Slot {
        self.heap.alloc(self.current, size).as_mut()
    }
    pub unsafe fn new_scope(&mut self, variable: Slot, name: Option<Symbol>) -> Self {
        let scope = self.alloc(std::mem::size_of::<Scope>());
        let scope_ref = &mut (*(scope as *mut Scope));
        scope_ref.head = ObjectHead {
            tag: ObjectTag::Opaque,
            moved: false,
        };
        scope_ref.name = name;
        scope_ref.pointer = variable;
        scope_ref.prev = Some(self.current);
        Self {
            heap: self.heap.clone(),
            current: NonNull::new(scope as *mut Scope).unwrap_unchecked(),
        }
    }
}
