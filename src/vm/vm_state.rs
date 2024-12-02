use std::{collections::HashSet, ptr::NonNull};

use super::{
    memory_manage::SemiSpaceMemory,
    object_model::{ObjectHead, ObjectTag, Slot, Symbol},
};

#[derive(Debug, Clone)]
pub struct VMState {
    pub accumulator: Slot,
    pub environment: Slot,
    pub stack: Slot,

    pub heap: SemiSpaceMemory,
    pub symbol_cache: HashSet<String>,
}

impl VMState {
    pub fn alloc(&mut self, size: usize) -> Option<Slot> {
        self.heap.alloc(size).map(|ptr| ptr.as_ptr())
    }

    pub unsafe fn alloc_with_gc(&mut self, size: usize) -> Slot {
        if let Some(r) = self.alloc(size) {
            return r;
        }
        self.gc();
        if let Some(r) = self.alloc(size) {
            return r;
        }
        panic!("OutOfMemory");
    }

    pub unsafe fn gc(&mut self) {
        self.heap.gc((
            &mut self.accumulator,
            &mut self.environment,
            &mut self.stack,
        ))
    }

    pub unsafe fn symbol_register(&mut self, s: &str) -> Slot {
        let r = &mut self.symbol_cache;
        let value = if let Some(r) = r.get(s) {
            r as *const String as *mut String
        } else {
            r.insert(s.to_owned());
            r.get(s).unwrap() as *const String as *mut String
        };

        let value = NonNull::new(value).unwrap();
        let r = self.alloc_with_gc(std::mem::size_of::<Symbol>());
        *(r as *mut Symbol) = Symbol {
            head: ObjectHead {
                __align32: 0,
                __align16: 0,
                tag: ObjectTag::Symbol,
                moved: false,
            },
            value,
        };
        return r;
    }
}
