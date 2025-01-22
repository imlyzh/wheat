use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

use wheat::vm::{
    make_object::make_null, memory_manage::SemiSpaceMemory,
    vm_state::VMState,
};

fn main() {
    println!("wheat programming language. Copyright 2024 lyzh");
    unsafe {
        let mut vms = VMState {
            heap: SemiSpaceMemory::init(200, false),
            hidden_class_cache: HashMap::new(),
            symbol_cache: HashMap::new(),
            accumulator:  make_null(),
            environment:  make_null(),
            stack: make_null(),
            current_codes: make_null(),
        };
    }
}
