use std::{collections::HashSet, ptr::NonNull};

use crate::{
    memory_manage::SemiSpaceMemory,
    object_model::{ObjectHead, ObjectTag, Slot, Symbol},
    scope_model::Scope,
};

#[derive(Debug, Clone)]
pub struct VMState {
    pub heap: SemiSpaceMemory,
    pub current: Option<NonNull<Scope>>,
    pub symbol_cache: HashSet<String>,
}

impl Drop for VMState {
    fn drop(&mut self) {}
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
        scope_ref.prev = self.current;
        Self {
            heap: self.heap.clone(),
            current: Some(NonNull::new(scope as *mut Scope).unwrap_unchecked()),
            symbol_cache: HashSet::new(),
        }
    }

    pub unsafe fn symbol_register(&mut self, s: &str) -> Slot {
        let r = &mut self.symbol_cache;
        let value = if let Some(r) = r.get(s) {
            r as *const String as *mut String
        } else {
            r.insert(s.to_owned());
            r.get(s).unwrap_unchecked() as *const String as *mut String
        };

        let value = NonNull::new(value).unwrap();
        let r = self.alloc(std::mem::size_of::<Symbol>());
        *(r as *mut Symbol) = Symbol {
            head: ObjectHead {
                tag: ObjectTag::Symbol,
                moved: false,
            },
            value,
        };
        return r;
    }
}
