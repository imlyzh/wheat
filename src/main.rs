use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

use wheat::vm::{
    make_object::make_null, memory_manage::SemiSpaceMemory, object_operator::read::read_,
    vm_state::VMState,
};

fn main() {
    println!("wheat programming language. Copyright 2024 lyzh");
    unsafe {
        let mut vms = VMState {
            heap: SemiSpaceMemory::init(200, false),
            symbol_cache: HashMap::new(),
            accumulator:  make_null(),
            environment:  make_null(),
            stack: make_null(),
            current_codes: make_null(),
        };
        loop {
            print!("> ");
            stdout().flush().unwrap();
            let mut input_buf = "".to_string();
            stdin().read_line(&mut input_buf).unwrap();
            let slot = read_(&mut vms, &input_buf);
            println!("slot pointer: {:?}", slot);
            println!("alloc counter: {:?}", vms.heap.alloc_count);
        }
    }
}
