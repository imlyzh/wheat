use std::collections::HashMap;


use super::{
    make_object::make_symbol, memory_manage::SemiSpaceMemory, object_model::{HiddenKlass, HiddenKlassHandle, Slot}
};

#[derive(Debug, Clone)]
pub struct VMState {
    pub accumulator: Slot,
    pub current_codes: Slot,
    pub environment: Slot,
    pub stack: Slot,

    pub heap: SemiSpaceMemory,
    pub symbol_cache: HashMap<String, Slot>,
    pub hidden_class_cache: HashMap<HiddenKlassHandle, *const HiddenKlass>
}

pub unsafe fn run(vms: &mut VMState) -> Slot {
    todo!()
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
        if let Some(r) = r.get(s) {
            *r
        } else {
            let r = make_symbol(self, s);
            self.symbol_cache.insert(s.to_owned(),            r         );
            r
        }
    }
}
